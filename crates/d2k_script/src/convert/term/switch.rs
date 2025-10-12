pub fn convert_switch(item: &crate::Pair) -> u32 {
    assert_eq!(item.as_rule(), crate::grammar::Rule::switch);
    item.as_str()[1..].parse().unwrap()
}
