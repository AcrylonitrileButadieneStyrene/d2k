use std::sync::Arc;

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

impl From<Atom> for crate::Expr {
    fn from(value: Atom) -> Self {
        crate::Expr::Atom(value)
    }
}
