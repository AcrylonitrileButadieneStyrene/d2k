#[derive(Debug, Default)]
pub struct Context {
    pub labels: Vec<std::sync::Arc<str>>,
}

impl Context {
    pub fn with(&mut self, pair: crate::Pair) {
        match pair.as_rule() {
            crate::grammar::Rule::label_word => self.labels.push(pair.as_str().into()),
            _ => {
                pair.into_inner().for_each(|pair| {
                    self.with(pair);
                });
            }
        }
    }
}
