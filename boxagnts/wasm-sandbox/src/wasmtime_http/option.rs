use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RunOption {
    /// Grant access of a host directory to guest root dir.
    pub work_dir: Option<String>,

    /// Grant access of a host directory to a guest.
    pub map_dirs: Option<HashMap<String, String>>,

    /// Pass an environment variable to the program.
    pub env_vars: Option<HashMap<String, Option<String>>>,

    /// The network destinations which the component is allowed to access.
    /// Each entry is in the form "(scheme)://(host)[:port]". Each element
    /// allows * as a wildcard e.g. "https://\*" (HTTPS on the default port
    /// to any destination) or "\*://localhost:\*" (any protocol to any port on
    /// localhost). The host part allows segment wildcards for subdomains
    /// e.g. "https://\*.example.com". Application variables are allowed using
    /// `{{ my_var }}`` syntax.
    pub allowed_outbound_hosts: Option<Vec<String>>,
    
    ///
    pub block_url: Option<String>,


    /// Set of IP networks to be blocked
    pub block_networks: Option<Vec<String>>,

    /// Pass a wasi config variable to the program.
    pub config_vars: Option<HashMap<String, String>>,

    /// Preset data for the In-Memory provider of WASI key-value API.
    pub keyvalue_vars: Option<HashMap<String, String>>,

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

    //
    /// The WebAssembly Component to run
    pub wasm_file: String,
}