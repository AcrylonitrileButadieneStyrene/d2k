use crate::{
    il2k::Expr,
    r2ks::{self, parse, read},
};

pub fn switch(parser: &mut parse::Parser, start: u32, end: u32) -> Result<Expr, r2ks::ParseError> {
    parser.expect(d2k_lexer::R2KSToken::AssignSet)?;

    Ok(Expr::List(vec![
        read::assign_switch_value(&parser.next())?,
        super::variable::single_or_range(start, end),
    ]))
}
