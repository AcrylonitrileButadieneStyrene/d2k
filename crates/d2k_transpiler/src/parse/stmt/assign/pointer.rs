use d2k_lexer::R2KSToken;

use crate::{Atom, ConvertError, Expr, parse, read};

pub fn pointer(parser: &mut parse::Parser, target: u32) -> Result<Expr, ConvertError> {
    let destination = Expr::List(vec![
        Expr::Atom(Atom::symbol("pointer")),
        Expr::Atom(Atom::U32(target)),
    ]);

    let operation_token = parser.next();
    let value_token = parser.next();

    Ok(Expr::List(
        if matches!(operation_token, R2KSToken::AssignSet)
            && let Ok(value) = read::assign_switch_value(&value_token)
        {
            vec![value, destination]
        } else {
            let operation = read::assign_op(operation_token)?;
            let value = read::assign_variable_value(parser, &value_token)?;
            vec![operation, destination, value]
        },
    ))
}
