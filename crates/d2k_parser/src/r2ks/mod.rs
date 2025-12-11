mod error;
mod expected;
mod parse;
pub mod read;

pub use error::ParseError;
pub use expected::Expected;

pub fn parse(
    tokens: Vec<(d2k_lexer::R2KSToken, std::ops::Range<usize>)>,
) -> Result<super::il2k::Expr, codespan_reporting::diagnostic::Diagnostic<()>> {
    let (tokens, spans) = tokens.iter().cloned().unzip();

    parse::Parser {
        tokens,
        spans,
        position: 0,
    }
    .run_to_completion()
    .map_err(|(mut parser, err)| match err {
        ParseError::Expected(expected) => codespan_reporting::diagnostic::Diagnostic::error()
            .with_message(format!(
                "Expected {} but found {:?}",
                match expected {
                    Expected::Single(x) => x,
                    Expected::Multiple(x) => x.join(", "),
                },
                parser.peek_back()
            ))
            .with_label(codespan_reporting::diagnostic::Label::primary(
                (),
                parser.spans[parser.position - 1].clone(),
            )),
    })
}
