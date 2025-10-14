use crate::grammar;

pub fn convert_term(item: crate::Pair) -> Term {
    let rule = item.as_rule();
    let inner = item.into_inner();
    match rule {
        grammar::Rule::switch => Term::Switch(inner.as_str().parse().unwrap()),
        grammar::Rule::variable => Term::Variable(inner.as_str().parse().unwrap()),
        grammar::Rule::event => Term::Event(inner.as_str().parse().unwrap()),
        grammar::Rule::common_event => Term::CommonEvent(inner.as_str().parse().unwrap()),
        x => panic!("tried to convert non-term {x:?}"),
    }
}

pub enum Term {
    Switch(u32),
    Variable(u32),
    Event(u32),
    CommonEvent(u32),
}

macro_rules! getter {
    ($name:ident, $variant:path) => {
        pub fn $name(&self) -> Option<u32> {
            match self {
                $variant(x) => Some(*x),
                _ => None,
            }
        }
    };
}

impl Term {
    getter!(switch, Self::Switch);
    getter!(variable, Self::Variable);
    getter!(event, Self::Event);
    getter!(common_event, Self::CommonEvent);
}
