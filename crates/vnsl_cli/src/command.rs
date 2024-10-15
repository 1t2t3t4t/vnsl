use crate::model::{CompileMode, CompileOptions};

mod compile;

pub fn handle_compile(mode: CompileMode, options: CompileOptions) -> anyhow::Result<()> {
    let input_dir = options.input.clone().unwrap_or("./".to_string());
    let compiled_source = vnsl_compiler::compile_dir(&input_dir)?;
    match mode {
        CompileMode::Json => compile::compile_json(compiled_source, options)?,
    }
    Ok(())
}
