use codespan_reporting::{
    diagnostic::Diagnostic,
    files::Files,
    term::{
        self, Config,
        termcolor::{ColorChoice, StandardStream},
    },
};

mod atom;
mod expr;
pub use atom::Atom;
pub use expr::Expr;

pub fn emit<'files, F>(
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<(), codespan_reporting::files::Error>
where
    F: Files<'files> + ?Sized,
{
    term::emit(
        &mut StandardStream::stderr(ColorChoice::Auto),
        &Config {
            ..Default::default()
        },
        files,
        diagnostic,
    )
}
