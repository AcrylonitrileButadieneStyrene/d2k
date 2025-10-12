pub fn convert_variable(item: &crate::Pair) -> u32 {
    assert_eq!(item.as_rule(), crate::grammar::Rule::variable);
    item.as_str()[1..].parse().unwrap()
}
