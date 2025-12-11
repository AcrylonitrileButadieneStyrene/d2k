#![feature(iterator_try_collect)]

use itertools::Itertools as _;

mod il2k;
mod r2ks;

pub use il2k::IL2KToken;
pub use r2ks::R2KSToken;

fn from_file<'a, T>(file_name: &str, source: &'a str) -> Vec<(T, std::ops::Range<usize>)>
where
    T: logos::Logos<'a, Source = str, Extras: Default, Error = ()>,
{
    let (tokens, errors): (Vec<_>, Vec<_>) = <T as logos::Logos>::lexer(source)
        .spanned()
        .partition_map(|(token, span)| match token {
            Ok(token) => itertools::Either::Left((token, span)),
            Err(()) => itertools::Either::Right(span),
        });

    if !errors.is_empty() {
        d2k_common::emit(
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
