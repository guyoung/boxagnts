use std::time::Duration;

use wasmtime::component::Component;
use wasmtime::{Engine, Result, Store, StoreContextMut, StoreLimits};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi_http::handler::{HandlerState, StoreBundle};

#[cfg(feature = "wasi-config")]
use wasmtime_wasi_config::WasiConfigVariables;
#[cfg(feature = "wasi-keyvalue")]
use wasmtime_wasi_keyvalue::WasiKeyValueCtxBuilder;

use crate::wasmtime_cli::common::RunCommon;

use super::host::Host;
use super::output::LogStream;
use super::output::Output;

pub struct HostHandlerState {
    pub run_common: RunCommon,
    pub engine: Engine,
    pub component: Component,
    pub max_instance_reuse_count: usize,
    pub max_instance_concurrent_reuse_count: usize,
    pub idle_instance_timeout: Duration,
    pub no_logging_prefix: bool,
}

impl HandlerState for HostHandlerState {
    type StoreData = Host;

    fn new_store(&self, req_id: Option<u64>) -> Result<StoreBundle<Host>> {
        let mut store = new_store(
            &self.run_common,
            &self.engine,
            req_id,
            self.no_logging_prefix,
        )?;
        let write_profile = setup_epoch_handler(&self.run_common, &mut store)?;

        Ok(StoreBundle {
            store,
            write_profile,
        })
    }

    fn request_timeout(&self) -> Duration {
        self.run_common.common.wasm.timeout.unwrap_or(Duration::MAX)
    }

    fn idle_instance_timeout(&self) -> Duration {
        self.idle_instance_timeout
    }

    fn max_instance_reuse_count(&self) -> usize {
        self.max_instance_reuse_count
    }

    fn max_instance_concurrent_reuse_count(&self) -> usize {
        self.max_instance_concurrent_reuse_count
    }

    fn handle_worker_error(&self, error: wasmtime::Error) {
        eprintln!("worker error: {error}");
    }
}

fn new_store(
    run_common: &RunCommon,
    engine: &Engine,
    req_id: Option<u64>,
    no_logging_prefix: bool,
) -> Result<Store<Host>> {
    let mut builder = WasiCtxBuilder::new();
    run_common.configure_wasip2(&mut builder)?;

    if let Some(req_id) = req_id {
        builder.env("REQUEST_ID", req_id.to_string());
    }

    let stdout_prefix: String;
    let stderr_prefix: String;
    match req_id {
        Some(req_id) if !no_logging_prefix => {
            stdout_prefix = format!("stdout [{req_id}] :: ");
            stderr_prefix = format!("stderr [{req_id}] :: ");
        }
        _ => {
            stdout_prefix = "".to_string();
            stderr_prefix = "".to_string();
        }
    }
    builder.stdout(LogStream::new(stdout_prefix, Output::Stdout));
    builder.stderr(LogStream::new(stderr_prefix, Output::Stderr));

    let mut table = wasmtime::component::ResourceTable::new();
    if let Some(max) = run_common.common.wasi.max_resources {
        table.set_max_capacity(max);
    }
    let mut host = Host {
        table,
        ctx: builder.build(),
        http: run_common.wasi_http_ctx()?,
        http_outgoing_body_buffer_chunks: run_common.common.wasi.http_outgoing_body_buffer_chunks,
        http_outgoing_body_chunk_size: run_common.common.wasi.http_outgoing_body_chunk_size,

        limits: StoreLimits::default(),

        #[cfg(feature = "wasi-config")]
        wasi_config: None,
        #[cfg(feature = "wasi-keyvalue")]
        wasi_keyvalue: None,
        #[cfg(feature = "component-model-async")]
        p3_http: crate::wasmtime_cli::common::DefaultP3Ctx,

        /*** ***/
        allowed_outbound_hosts: Some(crate::extension::net::parse_allowed_outbound_hosts(
            run_common.allowed_outbound_hosts.clone(),
        )),
        block_networks: Some(crate::extension::net::parse_block_networks(
            run_common.block_networks.clone(),
        )),
        /*** ***/
    };

    #[cfg(feature = "wasi-config")]
    {
        let vars = WasiConfigVariables::from_iter(
            run_common
                .common
                .wasi
                .config_var
                .iter()
                .map(|v| (v.key.clone(), v.value.clone())),
        );
        host.wasi_config.replace(vars);
    }

    #[cfg(feature = "wasi-keyvalue")]
    {
        let ctx = WasiKeyValueCtxBuilder::new()
            .in_memory_data(
                run_common
                    .common
                    .wasi
                    .keyvalue_in_memory_data
                    .iter()
                    .map(|v| (v.key.clone(), v.value.clone())),
            )
            .build();
        host.wasi_keyvalue.replace(ctx);
    }

    let mut store = Store::new(engine, host);

    if let Some(fuel) = run_common.common.wasi.hostcall_fuel {
        store.set_hostcall_fuel(fuel);
    }

    store.data_mut().limits = run_common.store_limits();
    store.limiter(|t| &mut t.limits);

    // If fuel has been configured, we want to add the configured
    // fuel amount to this store.
    if let Some(fuel) = run_common.common.wasm.fuel {
        store.set_fuel(fuel)?;
    }

    Ok(store)
}

type WriteProfile = Box<dyn FnOnce(StoreContextMut<Host>) + Send>;

fn setup_epoch_handler(run_common: &RunCommon, store: &mut Store<Host>) -> Result<WriteProfile> {
    // Profiling disabled but there's a global request timeout
    if run_common.common.wasm.timeout.is_some() {
        store.epoch_deadline_async_yield_and_update(1);
    }

    Ok(Box::new(|_store| {}))
}
