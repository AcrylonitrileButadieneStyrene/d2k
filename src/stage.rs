use std::path::PathBuf;

#[derive(Clone)]
pub enum Stage {
    Lex,
    Parse,
    Compile,
    Serialize,
}

#[derive(Default)]
pub struct Stages {
    pub lex: Option<PathBuf>,
    pub parse: Option<PathBuf>,
    pub compile: Option<PathBuf>,
    pub serialize: Option<PathBuf>,
}

impl From<Vec<(Stage, PathBuf)>> for Stages {
    fn from(values: Vec<(Stage, PathBuf)>) -> Self {
        let mut this = Self::default();
        for (stage, path) in values {
            if let Some(duplicate) = match stage {
                Stage::Lex => &mut this.lex,
                Stage::Parse => &mut this.parse,
                Stage::Compile => &mut this.compile,
                Stage::Serialize => &mut this.serialize,
            }
            .replace(path)
            {
                log::warn!("Duplicate output {} will be ignored", duplicate.display());
            }
        }
        this
    }
}

impl Stages {
    pub fn lex(&self) -> Option<&std::path::Path> {
        self.lex.as_deref().or_else(|| {
            if self.parse.is_none() {
                Some(&std::path::Path::new("-"))
            } else {
                None
            }
        })
    }

    pub fn compile(&self) -> Option<&std::path::Path> {
        self.compile.as_deref().or_else(|| {
            if self.serialize.is_none() {
                Some(&std::path::Path::new("-"))
            } else {
                None
            }
        })
    }
}
