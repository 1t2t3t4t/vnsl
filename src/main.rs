fn main() -> anyhow::Result<()> {
    let test_script = include_str!("./test.vnsl");
    let _scene = vnsl_compiler::compile(test_script)?;

    Ok(())
}
