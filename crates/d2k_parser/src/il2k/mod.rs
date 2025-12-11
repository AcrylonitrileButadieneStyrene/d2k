use std::{fmt::Write, sync::Arc};

#[derive(Debug)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

impl Expr {
    pub fn from_il(tokens: Vec<(d2k_lexer::IL2KToken, std::ops::Range<usize>)>) -> Option<Expr> {
        Self::from_il_recursive(&tokens, &mut 0)
    }

    fn from_il_recursive(
        tokens: &[(d2k_lexer::IL2KToken, std::ops::Range<usize>)],
        index: &mut usize,
    ) -> Option<Expr> {
        let mut exprs = Vec::new();
        loop {
            let token = tokens.get(*index);
            *index += 1;

            exprs.push(match token {
                Some((d2k_lexer::IL2KToken::Atom(atom), _)) => {
                    Expr::Atom(Atom::symbol(atom.as_ref()))
                }
                Some((d2k_lexer::IL2KToken::ParenOpen, _)) => {
                    Self::from_il_recursive(&tokens, index)?
                }
                Some((d2k_lexer::IL2KToken::ParenClose, _)) | None => {
                    return Some(Expr::List(exprs));
                }
            });
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => {
                write!(f, "{}", atom)?;
            }
            Self::List(list) => {
                f.write_char('(')?;
                for (index, expr) in list.iter().enumerate() {
                    if index != 0 {
                        f.write_char(' ')?;
                    }

                    write!(f, "{}", expr)?;
                }
                f.write_char(')')?;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum Atom {
    Symbol(Arc<str>),
    Label(Box<str>),
    Text(String),
    U32(u32),
    I32(i32),
    Bool(bool),
}

impl Atom {
    pub fn symbol(str: impl Into<Arc<str>>) -> Self {
        Self::Symbol(str.into())
    }

    pub fn label(str: impl Into<Box<str>>) -> Self {
        Self::Label(str.into())
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(symbol) => f.write_str(&symbol)?,
            Self::Label(label) => write!(f, "${}", label)?,
            Self::Text(text) => write!(f, "\"{}\"", text)?,
            Self::U32(value) => write!(f, "{}", value)?,
            Self::I32(value) => write!(f, "{}", value)?,
            Self::Bool(value) => write!(f, "{}", value)?,
        }

        Ok(())
    }
}
