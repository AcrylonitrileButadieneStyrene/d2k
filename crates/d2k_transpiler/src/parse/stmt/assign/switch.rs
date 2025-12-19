use crate::{ConvertError, Expr, parse::Parser, read};

pub fn switch(parser: &mut Parser, start: u32, end: u32) -> Result<Expr, ConvertError> {
    parser.expect(d2k_lexer::R2KSToken::AssignSet)?;

    Ok(Expr::List(vec![
        read::assign_switch_value(&parser.next())?,
        super::variable::single_or_range(start, end),
    ]))
}
