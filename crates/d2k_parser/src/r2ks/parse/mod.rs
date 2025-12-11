use crate::{
    il2k::{self, Expr},
    r2ks::{self, parse},
};

crate::export!(stmt);
crate::export!(ident);

#[derive(Debug)]
pub struct Parser {
    pub(crate) tokens: Vec<d2k_lexer::R2KSToken>,
    pub(crate) spans: Vec<std::ops::Range<usize>>,
    pub(crate) position: usize,
}

impl Parser {
    pub fn done(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn forward(&mut self) {
        if self.done() {
            panic!("out of bounds token read")
        }
        self.position += 1;
    }

    pub fn peek(&mut self) -> Option<d2k_lexer::R2KSToken> {
        self.tokens.get(self.position).cloned()
    }

    pub fn peek_back(&mut self) -> d2k_lexer::R2KSToken {
        self.tokens[self.position - 1].clone()
    }

    pub fn next(&mut self) -> d2k_lexer::R2KSToken {
        self.forward();
        self.peek_back()
    }

    pub fn run_to_completion(mut self) -> Result<il2k::Expr, (Box<Self>, r2ks::ParseError)> {
        let mut statements = Vec::new();
        while !self.done() {
            match parse::stmt(&mut self) {
                Ok(val) => statements.push(val),
                Err(err) => return Err((Box::new(self), err)),
            }
        }

        Ok(il2k::Expr::List(statements))
    }

    pub fn expect(&mut self, val: d2k_lexer::R2KSToken) -> Result<(), r2ks::ParseError> {
        let next = self.next();
        if next != val {
            Err(self.expected(format!("{val:?}")))
        } else {
            Ok(())
        }
    }

    pub fn expected(&self, val: String) -> r2ks::ParseError {
        r2ks::Expected::Single(val).into()
    }

    fn parse_block(&mut self, prefix: Option<Expr>) -> Result<Expr, r2ks::ParseError> {
        self.expect(d2k_lexer::R2KSToken::BraceOpen)?;

        let mut buf = Vec::new();
        if let Some(prefix) = prefix {
            buf.push(prefix);
        }

        while !matches!(self.peek(), Some(d2k_lexer::R2KSToken::BraceClose)) {
            buf.push(parse::stmt(self)?);
        }

        self.forward();
        Ok(Expr::List(buf))
    }
}
