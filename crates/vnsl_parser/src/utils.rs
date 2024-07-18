use std::collections::HashMap;

use pest::iterators::Pair;

use crate::Rule;

pub fn extract_inner(rule: Pair<Rule>, as_rule: Rule) -> Pair<Rule> {
    let mut inner = rule.into_inner();
    debug_assert_eq!(inner.len(), 1);
    let next = inner.next().unwrap();
    debug_assert_eq!(next.as_rule(), as_rule);
    next
}

pub fn extract_inners<const SIZE: usize>(
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
    assert_eq!(map.len(), rules.len(), "Map should contain same length");
    for rule in rules {
        assert!(map.contains_key(&rule), "Map should contain key {:?}", rule);
    }
}
