use std::path::PathBuf;
use std::time::Duration;

use wasmtime::component::Linker;
use wasmtime::{Engine, Result, Store, bail};
use wasmtime_wasi_http::handler::p2::bindings as p2;
use wasmtime_wasi_http::handler::{ProxyHandler, ProxyPre};

#[cfg(feature = "wasi-config")]
use wasmtime_wasi_config::WasiConfig;
#[cfg(feature = "wasi-keyvalue")]
use wasmtime_wasi_keyvalue::WasiKeyValue;

use crate::wasmtime_cli::common::Profile;
use crate::wasmtime_cli::common::RunCommon;
use crate::wasmtime_cli::common::RunTarget;

use super::host::Host;
use super::state::HostHandlerState;

const DEFAULT_WASIP3_MAX_INSTANCE_REUSE_COUNT: usize = 128;
const DEFAULT_WASIP2_MAX_INSTANCE_REUSE_COUNT: usize = 1;
const DEFAULT_WASIP3_MAX_INSTANCE_CONCURRENT_REUSE_COUNT: usize = 16;

pub fn crate_proxy_handler(
    run_common: &mut RunCommon,
    component: PathBuf,
    max_instance_reuse_count: Option<usize>,
    max_instance_concurrent_reuse_count: Option<usize>,
) -> Result<ProxyHandler<HostHandlerState>> {
    let mut config = run_common
        .common
        .config(use_pooling_allocator_by_default().unwrap_or(None))?;
    config.wasm_component_model(true);

    if run_common.common.wasm.timeout.is_some() {
        config.epoch_interruption(true);
    }

    match run_common.profile {
        Some(Profile::Native(s)) => {
            config.profiler(s);
        }
        Some(Profile::Guest { .. }) => {
            config.epoch_interruption(true);
        }
        None => {}
    }

    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    add_to_linker(run_common, &mut linker)?;

    let component = match run_common.load_module(&engine, &component)? {
        RunTarget::Core(_) => bail!("The serve command currently requires a component"),
        RunTarget::Component(c) => c,
    };

    let instance = linker.instantiate_pre(&component)?;
    #[cfg(feature = "component-model-async")]
    let instance = match wasmtime_wasi_http::p3::bindings::ServicePre::new(instance.clone()) {
        Ok(pre) => ProxyPre::P3(pre),
        Err(_) => ProxyPre::P2(p2::ProxyPre::new(instance)?),
    };
    #[cfg(not(feature = "component-model-async"))]
    let instance = ProxyPre::P2(p2::ProxyPre::new(instance)?);

    let max_instance_reuse_count = max_instance_reuse_count.unwrap_or_else(|| {
        if let ProxyPre::P3(_) = &instance {
            DEFAULT_WASIP3_MAX_INSTANCE_REUSE_COUNT
        } else {
            DEFAULT_WASIP2_MAX_INSTANCE_REUSE_COUNT
        }
    });

    let max_instance_concurrent_reuse_count = if let ProxyPre::P3(_) = &instance {
        max_instance_concurrent_reuse_count
            .unwrap_or(DEFAULT_WASIP3_MAX_INSTANCE_CONCURRENT_REUSE_COUNT)
    } else {
        1
    };

    let handler = ProxyHandler::new(
        HostHandlerState {
            run_common: run_common.clone(),
            engine,
            component,
            max_instance_reuse_count,
            max_instance_concurrent_reuse_count,
            idle_instance_timeout: Duration::from_secs(1),
            no_logging_prefix: false,
        },
        instance,
    );

    Ok(handler)
}

fn add_to_linker(run_common: &RunCommon, linker: &mut Linker<Host>) -> Result<()> {
    run_common.validate_p3_option()?;
    let cli = run_common.validate_cli_enabled()?;

    // Repurpose the `-Scli` flag of `wasmtime run` for `wasmtime serve`
    // to serve as a signal to enable all WASI interfaces instead of just
    // those in the `proxy` world. If `-Scli` is present then add all
    // `command` APIs and then additionally add in the required HTTP APIs.
    //
    // If `-Scli` isn't passed then use the `add_to_linker_async`
    // bindings which adds just those interfaces that the proxy interface
    // uses.
    if cli == Some(true) {
        run_common.add_wasmtime_wasi_to_linker(linker)?;
        wasmtime_wasi_http::add_only_http_to_linker_async(linker)?;
        #[cfg(feature = "component-model-async")]
        if run_common
            .common
            .wasi
            .p3
            .unwrap_or(crate::wasmtime_cli::common::P3_DEFAULT)
        {
            wasmtime_wasi_http::p3::add_to_linker(linker)?;
        }
    } else {
        wasmtime_wasi_http::add_to_linker_async(linker)?;
        #[cfg(feature = "component-model-async")]
        if run_common
            .common
            .wasi
            .p3
            .unwrap_or(crate::wasmtime_cli::common::P3_DEFAULT)
        {
            wasmtime_wasi_http::p3::add_to_linker(linker)?;
            wasmtime_wasi::p3::clocks::add_to_linker(linker)?;
            wasmtime_wasi::p3::random::add_to_linker(linker)?;
            wasmtime_wasi::p3::cli::add_to_linker(linker)?;
        }
    }

    #[cfg(feature = "wasi-config")]
    {
        wasmtime_wasi_config::add_to_linker(linker, |h| {
            WasiConfig::from(h.wasi_config.as_ref().unwrap())
        })?;
    }

    #[cfg(feature = "wasi-keyvalue")]
    {
        wasmtime_wasi_keyvalue::add_to_linker(linker, |h: &mut Host| {
            WasiKeyValue::new(h.wasi_keyvalue.as_ref().unwrap(), &mut h.table)
        })?;
    }

    Ok(())
}

/// The pooling allocator is tailor made for the `wasmtime serve` use case, so
/// try to use it when we can. The main cost of the pooling allocator, however,
/// is the virtual memory required to run it. Not all systems support the same
/// amount of virtual memory, for example some aarch64 and riscv64 configuration
/// only support 39 bits of virtual address space.
///
/// The pooling allocator, by default, will request 1000 linear memories each
/// sized at 6G per linear memory. This is 6T of virtual memory which ends up
/// being about 42 bits of the address space. This exceeds the 39 bit limit of
/// some systems, so there the pooling allocator will fail by default.
///
/// This function attempts to dynamically determine the hint for the pooling
/// allocator. This returns `Some(true)` if the pooling allocator should be used
/// by default, or `None` or an error otherwise.
///
/// The method for testing this is to allocate a 0-sized 64-bit linear memory
/// with a maximum size that's N bits large where we force all memories to be
/// static. This should attempt to acquire N bits of the virtual address space.
/// If successful that should mean that the pooling allocator is OK to use, but
/// if it fails then the pooling allocator is not used and the normal mmap-based
/// implementation is used instead.
fn use_pooling_allocator_by_default() -> Result<Option<bool>> {
    use wasmtime::{Config, Memory, MemoryType};
    const BITS_TO_TEST: u32 = 42;
    let mut config = Config::new();
    config.wasm_memory64(true);
    config.memory_reservation(1 << BITS_TO_TEST);
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());
    // NB: the maximum size is in wasm pages to take out the 16-bits of wasm
    // page size here from the maximum size.
    let ty = MemoryType::new64(0, Some(1 << (BITS_TO_TEST - 16)));
    if Memory::new(&mut store, ty).is_ok() {
        Ok(Some(true))
    } else {
        Ok(None)
    }
}
