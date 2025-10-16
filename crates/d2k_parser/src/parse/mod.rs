use crate::{parse, types};

crate::export!(stmt);
crate::export!(ident);

#[derive(Debug)]
pub struct Parser {
    pub(crate) tokens: Vec<d2k_lexer::Token>,
    pub(crate) spans: Vec<std::ops::Range<usize>>,
    pub(crate) position: usize,
    pub(crate) ast: crate::AST,
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

    pub fn peek(&mut self) -> d2k_lexer::Token {
        self.tokens[self.position].clone()
    }

    pub fn peek_back(&mut self) -> d2k_lexer::Token {
        self.tokens[self.position - 1].clone()
    }

    pub fn next(&mut self) -> d2k_lexer::Token {
        self.forward();
        self.peek_back()
    }

    pub fn run_to_completion(mut self) -> Result<crate::AST, (Box<Self>, crate::ParseError)> {
        while !self.done() {
            match parse::stmt(&mut self) {
                Ok(val) => self.ast.statements.push(val),
                Err(err) => return Err((Box::new(self), err)),
            }
        }

        Ok(self.ast)
    }

    pub fn expect(&mut self, val: d2k_lexer::Token) -> Result<(), crate::ParseError> {
        let next = self.next();
        if next != val {
            Err(self.expected(format!("{val:?}")))
        } else {
            Ok(())
        }
    }

    pub fn expected(&self, val: String) -> crate::ParseError {
        crate::Expected::Single(val).into()
    }

    fn parse_block(&mut self) -> Result<Vec<types::Statement>, crate::ParseError> {
        self.expect(d2k_lexer::Token::BraceOpen)?;

        let mut buf = Vec::new();
        while !matches!(self.peek(), d2k_lexer::Token::BraceClose) {
            buf.push(parse::stmt(self)?);
        }

        self.forward();
        Ok(buf)
    }
}
