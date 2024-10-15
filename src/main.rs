fn main() -> anyhow::Result<()> {
    let test_script = include_str!("./test.vnsl");
    let scene = vnsl_compiler::compile(test_script)?;

    println!("{:#?}", scene);
    Ok(())
}
