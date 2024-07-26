mod block;
mod data_type;
mod error;
mod label;
mod parser;
mod statement;
mod utils;

pub use parser::parse;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct VnslParser;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use std::fs;

    #[test]
    fn test_parsing_snapshot() {
        let dir = fs::read_dir("./snapshot").unwrap();
        let mut results = vec![];
        for entry in dir {
            let os_name = entry.unwrap().file_name();
            let file_name = os_name.to_str().unwrap();
            if file_name.ends_with(".vnsl") {
                let spec_name = file_name.trim_end_matches(".vnsl");
                results.push(snapshot(spec_name));
            }
        }

        assert!(results.iter().all(|b| b == &true))
    }

    fn snapshot(name: &str) -> bool {
        println!("Testing {name}");

        let script = fs::read_to_string(format!("./snapshot/{}.vnsl", name)).unwrap();
        let result = super::parse(&script).unwrap();

        let scn_str = format!("{:#?}", result).replace("\r\n", "\n");

        let expect_path = format!("./snapshot/{}.result", name);
        if fs::metadata(&expect_path).is_ok() {
            let expect = fs::read_to_string(expect_path)
                .unwrap()
                .replace("\r\n", "\n");
            assert_str_eq!(expect, scn_str);
            true
        } else {
            fs::write(expect_path, scn_str).unwrap();
            println!("Recording snapshot for {}", name);
            false
        }
    }
}
