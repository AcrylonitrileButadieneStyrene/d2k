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
}
