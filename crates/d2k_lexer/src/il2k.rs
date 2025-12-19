#[derive(Clone, Debug, PartialEq, Eq, logos::Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r";;[^\n]*")]
pub enum IL2KToken {
    #[regex(r#"("(\\"|[^"])*")|[^\s()]+"#, |lex| lex.slice().to_owned())]
    Atom(String),
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
}

impl IL2KToken {
    pub fn from_file(file_name: &str, source: &str) -> Vec<(Self, std::ops::Range<usize>)> {
        crate::from_file(file_name, source)
    }

    pub fn to_expr(tokens: &[(Self, std::ops::Range<usize>)]) -> Option<d2k_common::Expr> {
        to_expr_recursive(&tokens, &mut 0)
    }
}

fn to_expr_recursive(
    tokens: &[(IL2KToken, std::ops::Range<usize>)],
    index: &mut usize,
) -> Option<d2k_common::Expr> {
    let mut exprs = Vec::new();
    loop {
        let token = tokens.get(*index);
        *index += 1;

        exprs.push(match token {
            Some((IL2KToken::Atom(atom), _)) => {
                d2k_common::Expr::Atom(d2k_common::Atom::symbol(atom.as_ref()))
            }
            Some((IL2KToken::ParenOpen, _)) => to_expr_recursive(&tokens, index)?,
            Some((IL2KToken::ParenClose, _)) | None => {
                return Some(d2k_common::Expr::List(exprs));
            }
        });
    }
}
