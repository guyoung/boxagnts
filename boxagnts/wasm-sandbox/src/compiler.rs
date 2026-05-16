use wasmtime::{CodeBuilder, CodeHint, Engine, Result, bail, error::Context as _};


pub fn process(
    run_common: &mut crate::wasmtime_cli::common::RunCommon,
    wasm_file: &str,
    cache_dir: &str,
    cache_file: &str,
) -> Result<std::path::PathBuf> {
    let dir = std::path::Path::new(cache_dir);

    if !dir.exists() {
        std::fs::create_dir(&dir)?;
    }

    if !dir.is_dir() {
        bail!(
            "The wasm cache path ({}) must be a directory",
            dir.display()
        );
    }

    let cache_file = dir.join(cache_file);

    if cache_file.exists() {
        return Ok(cache_file);
    }
    
    
    let config = run_common.common.config(None)?;

    let engine = Engine::new(&config)?;

    let mut code = CodeBuilder::new(&engine);
    code.wasm_binary_or_text_file(&std::path::Path::new(wasm_file))?;


    let output_bytes = match code.hint() {
        #[cfg(feature = "component-model")]
        Some(CodeHint::Component) => code.compile_component_serialized()?,
        #[cfg(not(feature = "component-model"))]
        Some(CodeHint::Component) => {
            bail!("component model support was disabled at compile time")
        }
        Some(CodeHint::Module) | None => code.compile_module_serialized()?,
    };
    std::fs::write(&cache_file, output_bytes)
        .with_context(|| format!("failed to write output: {}", cache_file.display()))?;

    return Ok(cache_file);
}

