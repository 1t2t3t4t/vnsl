use std::io;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use vnsl_core::model::VnslScene;

mod fs;

pub fn compile_dir(path: &str) -> anyhow::Result<Vec<VnslScene>> {
    let scripts: io::Result<Vec<String>> = fs::scan_scripts(path, true)?
        .iter()
        .map(|p| std::fs::read_to_string(p))
        .collect();
    scripts?.par_iter().map(|src| compile(src)).collect()
}

pub fn compile(script: &str) -> anyhow::Result<VnslScene> {
    vnsl_parser::parse(script)
}

#[cfg(test)]
mod tests {
    use crate::compile_dir;

    #[test]
    fn test_compile_dir() -> anyhow::Result<()> {
        let result = compile_dir("./test_scripts")?;
        println!("{:#?}", result);
        Ok(())
    }
}
