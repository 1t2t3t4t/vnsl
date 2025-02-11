use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{debug, Rule};

mod macros;

pub fn extract_inner(rule: Pair<Rule>) -> Pair<Rule> {
    let mut inner = rule.into_inner();
    debug_assert_eq!(inner.len(), 1);
    inner.next().unwrap()
}

pub fn extract_inner_as_rule(rule: Pair<Rule>, as_rule: Rule) -> Pair<Rule> {
    let next = extract_inner(rule);
    debug_assert_eq!(next.as_rule(), as_rule);
    next
}

pub fn extract_inners_optional<const SIZE: usize>(
    rule: Pair<Rule>,
    inner_rules: [Rule; SIZE],
) -> HashMap<Rule, Pair<Rule>> {
    let mut map = HashMap::with_capacity(SIZE);
    for inner in rule.into_inner() {
        if inner_rules.contains(&inner.as_rule()) {
            map.insert(inner.as_rule(), inner);
        }
    }
    map
}

pub fn extract_inners<const SIZE: usize>(
    rule: Pair<Rule>,
    inner_rules: [Rule; SIZE],
) -> HashMap<Rule, Pair<Rule>> {
    let map = extract_inners_optional(rule, inner_rules);
    debug!({
        assert_eq!(
            map.len(),
            inner_rules.len(),
            "Map should contain same length"
        );
        for rule in inner_rules {
            assert!(map.contains_key(&rule), "Map should contain key {:?}", rule);
        }
    });
    map
}
