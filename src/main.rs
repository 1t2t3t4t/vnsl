fn main() -> anyhow::Result<()> {
    let test_script = include_str!("./test.vnsl");
    vnsl_compiler::compile(test_script)?;

    Ok(())
}
