use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct VnslParser;


#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::{Rule, VnslParser};

    #[test]
    fn test_parsing() {
        let example = include_str!("test.scene");
        let scene = VnslParser::parse(Rule::script, example)
            .unwrap()
            .next().unwrap();
        for i in scene.into_inner() {
            println!("{:?}", i.as_rule())
        }
    }
}