use anyhow::Ok;

pub fn compile(script: impl Into<String>) -> anyhow::Result<()> {
    let scene = vnsl_parser::parse(&script.into())?;
    println!("{:#?}", scene);
    Ok(())
}
