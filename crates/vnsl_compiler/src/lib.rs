use std::io;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use vnsl_core::model::VnslScene;

mod fs;

pub fn compile_dir(path: &str) -> anyhow::Result<Vec<VnslScene>> {
    let scripts: io::Result<Vec<String>> = fs::scan_scripts(path, true)?
        .iter()
        .map(std::fs::read_to_string)
        .collect();
    compile_scripts(&scripts?)
}

pub fn compile_scripts(scripts: &[String]) -> anyhow::Result<Vec<VnslScene>> {
    scripts.par_iter().map(|src| compile(src)).collect()
}

pub fn compile(script: &str) -> anyhow::Result<VnslScene> {
    // TODO: - Impl some validation of the script
    vnsl_parser::parse_scene(script)
}

#[cfg(test)]
mod tests {
    use crate::compile_dir;

    #[test]
    fn test_compile_dir() {
        compile_dir("./test_scripts").unwrap();
    }
}
