use vnsl_runtime::RuntimeDelegateHandler;

struct Handler;

impl RuntimeDelegateHandler for Handler {}

fn main() -> anyhow::Result<()> {
    let test_script = include_str!("../spec.vnsl");
    let scene = vnsl_compiler::compile(test_script)?;

    let mut runtime = vnsl_runtime::Runtime::new(scene);
    let handler = Handler;
    loop {
        let res = runtime.step(&handler);
        println!("{:?}", res);

        if matches!(res, Ok(vnsl_runtime::RuntimeCommand::EndOfStack)) {
            break;
        }
    }
    Ok(())
}
