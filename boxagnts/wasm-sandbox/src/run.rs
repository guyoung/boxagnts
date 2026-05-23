use std::ffi::OsString;
use std::time::Duration;

use bytes::Bytes;
use wasmtime::bail;

/// Env option key-value object



/// Run a WebAssembly Component configuration option
#[derive(Default, Debug)]
pub struct RunOption {
    /// Grant access of a host directory to guest root dir.
    ///
    /// Example: `--work-dir ./`
    pub work_dir: Option<String>,

    /// Grant access of a host directory to a guest.
    ///
    /// Example: `--map-dir ./usr/hello::/hello`
    pub map_dirs: Option<Vec<(String, String)>>,

    /// Pass an environment variable to the program.
    ///
    /// Example: `--env FOO=BAR`
    pub env_vars: Option<Vec<(String, Option<String>)>>,

    /// The network destinations which the component is allowed to access.
    /// Each entry is in the form "(scheme)://(host)[:port]". Each element
    /// allows * as a wildcard e.g. "https://\*" (HTTPS on the default port
    /// to any destination) or "\*://localhost:\*" (any protocol to any port on
    /// localhost). The host part allows segment wildcards for subdomains
    /// e.g. "https://\*.example.com". Application variables are allowed using
    /// `{{ my_var }}`` syntax.
    ///
    /// Example: `--allowed-outbound-host https://*.github.net`
    pub allowed_outbound_hosts: Option<Vec<String>>,
    
    ///
    pub block_url: Option<String>,

    /// Set of IP networks to be blocked
    ///
    /// Example: `--block-network 1.1.1.1/32  --block-network private`
    pub block_networks: Option<Vec<String>>,

    // wasm option
    /// Maximum execution time of wasm code before timing out (seconds)
    pub wasm_timeout: Option<u32>,

    /// Maximum size, in bytes, that a linear memory is allowed to reach.
    /// Growth beyond this limit will cause memory.grow instructions in
    /// WebAssembly Component to return -1 and fail.
    pub wasm_max_memory_size: Option<u32>,

    /// Maximum stack size, in bytes, that wasm is allowed to consume before
    /// a stack overflow is reported.
    pub wasm_max_wasm_stack: Option<u32>,

    /// Enable execution fuel with N units fuel, trapping after running out
    /// of fuel.
    ///
    /// Most WebAssembly instructions consume 1 unit of fuel. Some
    /// instructions, such as `nop`, `drop`, `block`, and `loop`, consume 0
    /// units, as any execution cost associated with them involves other
    /// instructions which do consume fuel.
    pub wasm_fuel: Option<u32>,

    //
    /// Precompiled WebAssembly Component as `*.cwasm` files cache dir.
    pub wasm_cache_dir: Option<String>,
}

pub async fn execute(
    wasm_file: String,
    invoke: Option<String>,
    args: Option<Vec<String>>,
    option: RunOption,
    program_name: Option<String>,
) -> wasmtime::Result<(Bytes, Bytes)> {
    let mut dirs: Vec<(String, String)> = Vec::new();
    if let Some(dir) = option.work_dir {
        dirs.push((dir, "/".to_string()));
    }
    if let Some(map_dirs) = option.map_dirs {
        for (k, v) in map_dirs {
            dirs.push((k, v));
        }
    }

    let mut vars: Vec<(String, Option<String>)> = Vec::new();
    if let Some(env_vars) = option.env_vars {
        for (k, v) in env_vars {
            vars.push((k, v));
        }
    }

    let mut run_common = crate::wasmtime_cli::common::RunCommon {
        common: Default::default(),
        allow_precompiled: false,
        profile: None,
        dirs,
        vars,
        allowed_outbound_hosts: option.allowed_outbound_hosts.unwrap_or(Vec::new()),
        block_url: option.block_url.clone(),
        block_networks: option.block_networks.unwrap_or(Vec::new()),
    };
    run_common.common.wasm.component_model = Some(true);

    // C++ exception handle: wasm.exceptions
    run_common.common.wasm.exceptions = Some(true);

    run_common.common.wasi.cli = Some(true);
    run_common.common.wasi.http = Some(true);
    run_common.common.wasi.inherit_network = Some(true);
    run_common.common.wasi.allow_ip_name_lookup = Some(true);
    run_common.common.wasi.tcp = Some(true);
    run_common.common.wasi.udp = Some(true);

    let timeout = if let Some(timeout) = option.wasm_timeout {
        Some(Duration::from_secs(timeout as u64))
    } else {
        None
    };

    run_common.common.wasm.timeout = timeout;
    run_common.common.wasm.max_memory_size = option.wasm_max_memory_size.map(|v| v as usize);
    run_common.common.wasm.max_wasm_stack = option.wasm_max_wasm_stack.map(|v| v as usize);
    run_common.common.wasm.fuel = option.wasm_fuel.map(|v| v as u64);

    let mut wasm_file = wasm_file;
    let filename = get_filename(&wasm_file);
    let wasm_file_md5 = check_file(&wasm_file)?;
    let wasm_cache_file = format!("{}.cwasm", wasm_file_md5);

    if let Some(dir) = option.wasm_cache_dir {
        if let Ok(cache_file) =
            crate::compiler::process(&mut run_common, &wasm_file, &dir, &wasm_cache_file)
        {
            run_common.allow_precompiled = true;
            wasm_file = cache_file.to_string_lossy().to_string();
        }
    }

    let mut module_and_args: Vec<OsString> = Vec::new();
    module_and_args.push(wasm_file.into());

    if let Some(args) = args {
        for arg in args {
            module_and_args.push(arg.clone().into());
        }
    }

    let command = crate::wasmtime_cli::commands::RunCommand {
        run: run_common,
        invoke,
        preloads: Default::default(),
        argv0: Some(program_name.unwrap_or(filename)),
        module_and_args,
    };

    command.execute().await

}

fn get_filename(path: &str) -> String {
    let path = std::path::Path::new(path);

    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    file_name.split('.').next().unwrap_or("").to_string()
}

fn check_file(path: &str) -> wasmtime::Result<String> {
    use std::io::Read;

    let file = std::fs::File::open(path);

    if let Err(e) = file {
        bail!("Error opening wasm file: {e}");
    }

    let mut file = file.unwrap();

    let mut context = md5::Context::new();
    let mut buffer = [0u8; 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        context.consume(&buffer[..n]);
    }

    let digest = context.finalize();

    Ok(format!("{:x}", digest))
}
