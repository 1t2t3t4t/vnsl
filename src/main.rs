fn main() -> anyhow::Result<()> {
    let test_script = include_str!("../spec.vnsl");
    let scene = vnsl_compiler::compile(test_script)?;

    let mut runtime = vnsl_runtime::Runtime::new();
    runtime.load_scene(scene);

    loop {
        let res = runtime.step().unwrap();
        println!("{:#?}", res);

        if matches!(res, vnsl_runtime::RuntimeCommand::EndOfScene) {
            break;
        }
    }
    Ok(())
}
