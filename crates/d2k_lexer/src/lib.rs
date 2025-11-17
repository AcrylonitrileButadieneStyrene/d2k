#![feature(iterator_try_collect)]

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, logos::Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"#[^\n]*")]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("break")]
    Break,
    #[token("return")]
    Return,
    #[token("erase")]
    #[token("delete")]
    #[token("destroy")]
    Destroy,
    #[regex("[rR]and(om)?")]
    Random,
    #[token("goto")]
    #[token("jump")]
    GoTo,

    #[token("=")]
    AssignSet,
    #[token("+=")]
    AssignAdd,
    #[token("-=")]
    AssignSub,
    #[token("*=")]
    AssignMul,
    #[token("/=")]
    AssignDiv,
    #[token("%=")]
    AssignMod,

    #[token("==")]
    Eq,
    #[token(">=")]
    Ge,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("!=")]
    Ne,

    // symbols
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token(",")]
    Comma,

    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*:", |lex| let str = lex.slice(); str[1..str.len() - 1].to_string())]
    Label(String),

    // values
    #[token("true")]
    #[token("on")]
    True,
    #[token("false")]
    #[token("off")]
    False,
    #[token("toggle")]
    Toggle,
    #[regex(r"[-+]?[0-9]+([\.,_][0-9]+)*", |lex| lex.slice().chars().filter(|char| *char != ',' && *char != '_'&& *char != '.').collect::<String>().parse().ok())]
    Number(i32),
    #[regex(r"[vV][0-9]{4}~[vV][0-9]{4}", |lex| let str = lex.slice(); str[1..5].parse().ok().zip(str[7..].parse().ok()))]
    VariableRange((u32, u32)),
    #[regex(r"[sS][0-9]{4}~[sS][0-9]{4}", |lex| let str = lex.slice(); str[1..5].parse().ok().zip(str[7..].parse().ok()))]
    SwitchRange((u32, u32)),
    #[regex(r"[vV][0-9]{4}", |lex| lex.slice()[1..].parse().ok())]
    Variable(u32),
    #[regex(r"[sS][0-9]{4}", |lex| lex.slice()[1..].parse().ok())]
    Switch(u32),
    #[regex(r"\[[vV][0-9]{4}\]", |lex| let str = lex.slice(); str[2..str.len() - 1].parse().ok())]
    Pointer(u32),
    #[regex(r"[eE][vV]?[0-9]{4}", |lex| trim(lex.slice()).parse().ok())]
    Event(u32),
    #[regex(r"[cC][eE][vV]?[0-9]{4}", |lex| trim(lex.slice()).parse().ok())]
    CommonEvent(u32),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| let str = lex.slice(); str[1..str.len() - 1].to_owned() )]
    String(String),
    #[regex(r";[^\n]*", |lex| lex.slice()[1..].trim().to_owned())]
    Comment(String),
    #[regex(r"\[[0-9]+\]", |lex| let str = lex.slice(); str[1..str.len() - 1].parse().ok())]
    Index(u32),
}

fn trim(val: &str) -> &str {
    &val[val.find(|c: char| c.is_ascii_digit()).unwrap()..]
}

type Range = std::ops::Range<usize>;

pub fn lex(file_name: &str, source: &str) -> Vec<(Token, Range)> {
    let (tokens, errors): (Vec<_>, Vec<_>) = <Token as logos::Logos>::lexer(source)
        .spanned()
        .partition_map(|(token, span)| match token {
            Ok(token) => itertools::Either::Left((token, span)),
            Err(()) => itertools::Either::Right(span),
        });

    if !errors.is_empty() {
        d2k_errors::emit(
            &codespan_reporting::files::SimpleFile::new(file_name, source),
            &codespan_reporting::diagnostic::Diagnostic::error()
                .with_message("Unrecognized token")
                .with_labels(
                    errors
                        .into_iter()
                        .map(|span| codespan_reporting::diagnostic::Label::primary((), span))
                        .collect(),
                ),
        )
        .unwrap();
        std::process::exit(1);
    } else {
        tokens
    }
}
