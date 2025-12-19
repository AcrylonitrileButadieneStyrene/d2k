use std::fmt::Write;

#[derive(Debug)]
pub enum Expr {
    Atom(crate::Atom),
    List(Vec<Expr>),
}

impl Expr {
    pub fn append(self, item: Self) -> Self {
        match self {
            Self::Atom(_) => Self::List(vec![self, item]),
            Self::List(mut vec) => {
                vec.push(item);
                Self::List(vec)
            }
        }
    }

    pub fn append_opt(self, item: Option<Self>) -> Self {
        if let Some(item) = item {
            self.append(item)
        } else {
            self
        }
    }

    pub fn to_list(self) -> Self {
        match self {
            Self::Atom(_) => Self::List(vec![self]),
            Self::List(_) => self,
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(atom) => {
                write!(f, "{}", atom)?;
            }
            Self::List(list) => {
                f.write_char('(')?;
                for (index, expr) in list.iter().enumerate() {
                    if index != 0 {
                        f.write_char(' ')?;
                    }

                    write!(f, "{}", expr)?;
                }
                f.write_char(')')?;
            }
        };

        Ok(())
    }
}
