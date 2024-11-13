pub struct Yellow {}
pub struct Blue {}
pub struct Purple {}
pub struct Green {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Yellow,
    Blue,
    Purple,
    Green,
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &'static str {
        match self {
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::Green => "green",
        }
    }
}

#[derive(Default)]
pub struct ColorIter(Option<Color>);

impl Iterator for ColorIter {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(Color::Green) => None,
            None => Some(Color::Yellow),
            Some(Color::Yellow) => Some(Color::Blue),
            Some(Color::Blue) => Some(Color::Purple),
            Some(Color::Purple) => Some(Color::Green),
        }
    }
}

impl Color {
    pub const fn from_int(int: u8) -> Self {
        match int {
            0 => Color::Yellow,
            1 => Color::Blue,
            2 => Color::Purple,
            3 => Color::Green,
            _ => unreachable!(),
        }
    }
}

pub trait AsColor {
    fn color() -> Color;
}

impl AsColor for Yellow {
    fn color() -> Color {
        Color::Yellow
    }
}

impl AsColor for Blue {
    fn color() -> Color {
        Color::Blue
    }
}

impl AsColor for Purple {
    fn color() -> Color {
        Color::Purple
    }
}

impl AsColor for Green {
    fn color() -> Color {
        Color::Green
    }
}
