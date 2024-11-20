#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Yellow = 0,
    Blue = 1,
    Purple = 2,
    Green = 3,
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
