use anyhow::Ok;

pub fn compile(script: &str) -> anyhow::Result<()> {
    let _ = vnsl_parser::parse(script).expect("Success");
    Ok(())
}
