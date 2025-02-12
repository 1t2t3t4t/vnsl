use std::time;

fn elapsed<T>(label: &str, f: impl FnOnce() -> T) -> T {
    let start = time::Instant::now();
    let res = f();
    println!(
        "Elapsed time {}: {} us.",
        label,
        start.elapsed().as_micros()
    );
    res
}

fn main() -> anyhow::Result<()> {
    let test_script = include_str!("../spec.vnsl");
    let scene = elapsed("compile", || vnsl_compiler::compile(test_script))?;

    let mut runtime = vnsl_runtime::Runtime::default();
    runtime.load_scene(scene);

    runtime.step().unwrap();
    runtime.step().unwrap();
    runtime.step().unwrap();

    let snapshot = elapsed("snapshot", || runtime.snapshot());
    let val = serde_json::to_string_pretty(&snapshot).unwrap();
    std::fs::write("./snapshot.json", &val).unwrap();
    Ok(())
}
