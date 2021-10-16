#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum Color {
    Yellow,
    Green,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Color::Green => Color::Yellow,
            Color::Yellow => Color::Green,
        }
    }
}
