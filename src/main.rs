use std::time;

use vnsl_runtime::scene_runner::SceneRunner;

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
    let test_script = include_str!("./test.vnsl");
    let scene = elapsed("compile", || vnsl_compiler::compile(test_script))?;

    let mut runner = SceneRunner::default();
    let result = runner.run_scene(scene.clone());
    std::fs::write("./scene_play", &result)?;

    let mut runtime = vnsl_runtime::Runtime::default();
    runtime.load_scene(scene);

    let snapshot = elapsed("snapshot", || runtime.snapshot());
    let val = serde_json::to_string_pretty(&snapshot)?;
    std::fs::write("./snapshot.json", &val)?;

    let encoded = snapshot.encode()?;
    std::fs::write("./snapshot_encoded", encoded)?;
    Ok(())
}
