use anyhow::Ok;

mod fs;

pub fn compile_dir(path: &str) -> anyhow::Result<()> {
    fs::scan_scripts(path, true)?;
    Ok(())
}

pub fn compile(script: &str) -> anyhow::Result<()> {
    let _ = vnsl_parser::parse(script).expect("Success");
    Ok(())
}
