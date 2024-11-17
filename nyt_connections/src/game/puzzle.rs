use crate::wasm_bindgen;

use super::color::AsColor;
use super::color::Blue;
use super::color::Color;
use super::color::Green;
use super::color::Purple;
use super::color::Yellow;
use super::Board;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use std::array;
use std::marker::PhantomData;
use thiserror::Error;

use flate2::write::GzDecoder;

use flate2::Compression;
use std::io::Write;

#[allow(unused_imports)]
use web_sys::console;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionPuzzle {
    yellow: ConnectionSet<Yellow>,
    blue: ConnectionSet<Blue>,
    purple: ConnectionSet<Purple>,
    green: ConnectionSet<Green>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionSet<Color: AsColor> {
    pub theme: String,
    pub words: [String; 4],
    color: PhantomData<Color>,
}

impl<C: AsColor> ConnectionSet<C> {
    fn new(theme: &str, words: [&str; 4]) -> Self {
        let words: [String; 4] = array::from_fn(|index| String::from(words[index]));
        let color = PhantomData;
        Self {
            theme: theme.into(),
            words,
            color,
        }
    }

    const fn empty_set() -> Self {
        Self {
            theme: String::new(),
            words: [const { String::new() }; 4],
            color: PhantomData,
        }
    }

    pub fn theme(&self) -> &str {
        &self.theme
    }

    pub fn words(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.words[0], self.words[1], self.words[2], self.words[3]
        )
    }
}

const fn empty_pair() -> (String, [String; 4]) {
    (String::new(), [const { String::new() }; 4])
}

impl ConnectionPuzzle {
    pub const fn empty() -> Self {
        let yellow = ConnectionSet::empty_set();
        let blue = ConnectionSet::empty_set();
        let purple = ConnectionSet::empty_set();
        let green = ConnectionSet::empty_set();

        Self {
            yellow,
            blue,
            purple,
            green,
        }
    }

    pub fn new(
        yellow: (&str, [&str; 4]),
        blue: (&str, [&str; 4]),
        purple: (&str, [&str; 4]),
        green: (&str, [&str; 4]),
    ) -> Self {
        let yellow = ConnectionSet::new(yellow.0, yellow.1);
        let blue = ConnectionSet::new(blue.0, blue.1);
        let purple = ConnectionSet::new(purple.0, purple.1);
        let green = ConnectionSet::new(green.0, green.1);

        Self {
            yellow,
            blue,
            purple,
            green,
        }
    }

    pub fn encode(&self) -> String {
        let postcard_bytes: Vec<u8> = postcard::to_allocvec(&self).expect("error serializing");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&postcard_bytes).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        URL_SAFE.encode(&compressed_bytes)
    }

    pub fn decode(code: &str) -> Result<Self, TranscodingError> {
        let compressed_bytes = URL_SAFE
            .decode(code)
            .map_err(|_| TranscodingError::Base64)?;

        let mut decoder = GzDecoder::new(Vec::new());
        decoder
            .write_all(&compressed_bytes[..])
            .map_err(|_| TranscodingError::Gzip)?;

        let postcard_bytes = decoder.finish().unwrap();
        postcard::from_bytes(&postcard_bytes[..]).map_err(|_| TranscodingError::Postcard)
    }

    pub fn all_keys(&self) -> [PuzzleKey; 16] {
        array::from_fn(|index| self.nth(index))
    }

    fn nth(&self, index: usize) -> PuzzleKey {
        assert!(index < 16);
        let color = Color::from_int((index / 4) as u8);
        //console::log_1(&format!("index {}", index).into());
        //console::log_1(&format!("mod 4:{}", index % 4).into());
        PuzzleKey::new(color, index % 4)
    }

    pub const fn yellow(&self) -> &ConnectionSet<Yellow> {
        &self.yellow
    }

    pub const fn blue(&self) -> &ConnectionSet<Blue> {
        &self.blue
    }

    pub const fn purple(&self) -> &ConnectionSet<Purple> {
        &self.purple
    }

    pub const fn green(&self) -> &ConnectionSet<Green> {
        &self.green
    }
}

#[wasm_bindgen]
#[derive(Debug, Error)]
pub enum TranscodingError {
    #[error("couldn't decode")]
    Base64,
    #[error("couldn't decompress")]
    Gzip,
    #[error("couldn't deserialize")]
    Postcard,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PuzzleKey {
    pub color: Color,
    pub word_index: usize,
}

impl PuzzleKey {
    fn new(color: Color, word_index: usize) -> Self {
        Self { color, word_index }
    }
}

impl Default for PuzzleKey {
    fn default() -> Self {
        Self {
            color: Color::Yellow,
            word_index: 0,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum CardState {
    Selected,
    Normal,
    Matched,
}

pub struct Card<'a> {
    pub color: Color,
    pub word: &'a str,
    pub theme: &'a str,
    pub state: CardState,
}

impl<'a> Card<'a> {
    pub fn from_key(key: &PuzzleKey, board: &'a Board) -> Self {
        let (word, theme) = match key.color {
            Color::Yellow => {
                let set = board.puzzle.yellow();
                (&set.words[key.word_index], &set.theme)
            }
            Color::Blue => {
                let set = board.puzzle.blue();
                (&set.words[key.word_index], &set.theme)
            }
            Color::Green => {
                let set = board.puzzle.green();
                (&set.words[key.word_index], &set.theme)
            }
            Color::Purple => {
                let set = board.puzzle.purple();
                (&set.words[key.word_index], &set.theme)
            }
        };

        let state = if board.selection.contains(key) {
            CardState::Selected
        } else if board.matched_cards.contains(&key.color) {
            CardState::Matched
        } else {
            CardState::Normal
        };

        Self {
            color: key.color,
            word,
            theme,
            state,
        }
    }

    pub fn background_color(&self) -> &str {
        match self.state {
            CardState::Normal => "var(--connections-light-beige)",
            CardState::Selected => "var(--connections-darker-beige)",
            //panic!("tried to render matched card")
            CardState::Matched => match self.color {
                Color::Yellow => "var(--connections-yellow)",
                Color::Green => "var(--connections-green)",
                Color::Blue => "var(--connections-blue)",
                Color::Purple => "var(--connections-maroon)",
            },
        }
    }

    pub fn text_color(&self) -> &str {
        match self.state {
            CardState::Selected => "white",
            _ => "black",
        }
    }

    pub fn class_name(&self) -> &str {
        match self.state {
            CardState::Normal => "card",
            CardState::Selected => "selected",
            CardState::Matched => match self.color {
                Color::Yellow => "matched_yellow",
                Color::Green => "matched_green",
                Color::Blue => "matched_blue",
                Color::Purple => "matched_purple",
            },
        }
    }
}

impl Default for ConnectionPuzzle {
    fn default() -> Self {
        let purple = ("___Room", ["war", "bed", "situation", "clean"]);
        let green = (
            "Domains of Greek Gods",
            ["victory", "ocean", "thunder", "music"],
        );
        let yellow = ("Minecraft Cake Recipe", ["wheat", "milk", "eggs", "sugar"]);
        let blue = ("noble gasses", ["helium", "argon", "krypton", "neon"]);

        Self::new(yellow, blue, purple, green)
    }
}
