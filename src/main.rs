use std::time;

fn elapsed<T>(f: impl FnOnce() -> T) -> T {
    let start = time::Instant::now();
    let res = f();
    println!("Elapsed time: {} us.", start.elapsed().as_micros());
    res
}

fn main() -> anyhow::Result<()> {
    let test_script = include_str!("../spec.vnsl");
    let scene = elapsed(|| vnsl_compiler::compile(test_script))?;

    let mut runtime = vnsl_runtime::Runtime::new();
    runtime.load_scene(scene);

    loop {
        let res = runtime.step().unwrap();
        println!("{:?}", res);

        if runtime.scene_ended() {
            break;
        }
    }
    Ok(())
}
