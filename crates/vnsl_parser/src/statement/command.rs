use anyhow::Ok;
use pest::iterators::Pair;

use crate::{model::VnslCommand, Rule};

mod action;
mod dialogue;
mod jump;

pub fn parse_command(rule: Pair<Rule>) -> anyhow::Result<VnslCommand> {
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::dialogue => Ok(dialogue::parse_dialogue(inner)),
        Rule::action => action::parse_action(inner),
        Rule::jump => jump::parse_jump(inner),
        Rule::var => Ok(VnslCommand::Var(var::parse_var(inner)?)),
        _ => unreachable!(),
    }
}

mod var {
    use std::collections::HashMap;

    use pest::iterators::Pair;

    use crate::{data_type, model::VnslVar, Rule};

    pub fn parse_var(rule: Pair<Rule>) -> anyhow::Result<VnslVar> {
        let mut pairs = extract_inner(rule, [Rule::identifier, Rule::data_type]);
        let name = pairs[&Rule::identifier].as_str().to_string();
        let value = data_type::parse_data_type(pairs.remove(&Rule::data_type).unwrap())?;
        Ok(VnslVar { name, value })
    }

    fn extract_inner<const SIZE: usize>(
        rule: Pair<Rule>,
        inner_rules: [Rule; SIZE],
    ) -> HashMap<Rule, Pair<Rule>> {
        let mut map = HashMap::with_capacity(SIZE);
        for inner in rule.into_inner() {
            if inner_rules.contains(&inner.as_rule()) {
                map.insert(inner.as_rule(), inner);
            }
        }

        ensure_map(&map, &inner_rules);
        map
    }

    #[cfg(debug_assertions)]
    fn ensure_map(map: &HashMap<Rule, Pair<Rule>>, rules: &[Rule]) {
        for rule in rules {
            assert!(map.contains_key(&rule), "Map should contain key {:?}", rule);
        }
    }
}
