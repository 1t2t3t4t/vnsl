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
    let mut test_script = "scene TestScript\n".to_string();
    for i in 0..10_000 {
        test_script.push_str(&format!("\"Line number {}\"\n", i));
    }
    std::fs::write("./test.vnsl", &test_script)?;

    elapsed("parse scene", || vnsl_parser::parse_scene(&test_script))?;
    elapsed("parse scene name", || {
        vnsl_parser::parse_scene_name(&test_script)
    })?;
    Ok(())
}
