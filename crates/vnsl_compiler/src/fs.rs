use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum FsError {
    #[error("Path '{0:?}' contains invalid character")]
    PathIsNotValidUnicode(PathBuf),
}

fn path_to_str(path_buf: &PathBuf, use_absolute: bool) -> anyhow::Result<String> {
    let path = if use_absolute {
        &std::path::absolute(path_buf)?
    } else {
        path_buf
    };
    let res = path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| FsError::PathIsNotValidUnicode(path_buf.clone()))?;
    Ok(res)
}

pub fn scan_scripts(path: &str, use_absolute: bool) -> anyhow::Result<Vec<String>> {
    let dir = std::fs::read_dir(path)?;
    let mut result = vec![];
    for entry in dir {
        let path = entry?.path();
        if path.extension().map_or(false, |e| e == "vnsl") {
            let path_str = path_to_str(&path, use_absolute)?;
            result.push(path_str);
        } else if path.metadata()?.is_dir() {
            let mut sub_result = scan_scripts(&path_to_str(&path, use_absolute)?, use_absolute)?;
            result.append(&mut sub_result);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::fs::scan_scripts;

    macro_rules! path_str {
        ($f:expr, $($str:expr),*) => {
            Path::new($f)$(.join($str))*.to_str().unwrap().to_string()
        };
    }

    #[test]
    fn test_compile_dir() {
        let mut res = scan_scripts("./test_scripts", false).unwrap();
        res.sort();
        assert_eq!(
            res,
            vec![
                path_str!("./test_scripts", "A.vnsl"),
                path_str!("./test_scripts", "B.vnsl"),
                path_str!("./test_scripts", "sub_folder", "D.vnsl"),
                path_str!("./test_scripts", "sub_folder", "sub_sub", "C.vnsl"),
            ]
        )
    }
}
