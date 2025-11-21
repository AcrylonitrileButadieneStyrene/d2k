#[derive(Debug, serde::Deserialize)]
pub struct Event {
    pub name: Option<String>,
    pub x: Position,
    pub y: Position,
    pub pages: Vec<Page>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Position {
    Constant(u32),
    Range(u32, u32),
}

#[derive(Debug, serde::Deserialize)]
pub struct Page {
    pub file: String,
    #[serde(default)]
    pub graphic: Graphic,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Graphic {
    pub direction: lcf::enums::Direction,
}

impl From<&Graphic> for lcf::lmu::event::page::Graphic {
    fn from(value: &Graphic) -> Self {
        let mut graphic = Self::default();
        graphic.direction = value.direction;
        graphic
    }
}
