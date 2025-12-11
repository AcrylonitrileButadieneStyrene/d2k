use crate::{
    il2k::{Atom, Expr},
    r2ks::{self, parse, read},
};

pub fn variable(
    parser: &mut parse::Parser,
    start: u32,
    end: u32,
) -> Result<Expr, r2ks::ParseError> {
    Ok(Expr::List(vec![
        read::assign_op(parser.next())?,
        single_or_range(start, end),
        {
            let token = parser.next();
            read::assign_variable_value(parser, &token)?
        },
    ]))
}

pub fn single_or_range(start: u32, end: u32) -> Expr {
    Expr::List(if start == end {
        vec![
            Expr::Atom(Atom::symbol("single")),
            Expr::Atom(Atom::U32(start)),
        ]
    } else {
        vec![
            Expr::Atom(Atom::symbol("range")),
            Expr::Atom(Atom::U32(start)),
            Expr::Atom(Atom::U32(end)),
        ]
    })
}
