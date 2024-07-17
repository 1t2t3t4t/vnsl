use anyhow::Ok;

pub fn compile(script: impl Into<String>) -> anyhow::Result<()> {
    let _ = vnsl_parser::parse(&script.into()).expect("Success");
    Ok(())
}
