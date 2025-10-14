mod switch;
mod variable;

use crate::grammar;

pub fn convert_assignment(assignment: crate::Pair) -> Vec<crate::Inst> {
    let rule = assignment.as_rule();
    let inner = assignment.into_inner();
    match rule {
        grammar::Rule::assignment_variable => variable::convert_assignment_variable(inner),
        grammar::Rule::assignment_switch => switch::convert_assignment_switch(inner),
        _ => unreachable!(),
    }
}
