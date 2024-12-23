use super::color::Color;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use flate2::write::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::ops::Deref;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use web_sys::console;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionPuzzle {
    yellow: YellowSet,
    blue: BlueSet,
    purple: PurpleSet,
    green: GreenSet,
}

#[wasm_bindgen]
#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlueSet(ConnectionSet);

#[wasm_bindgen]
#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct YellowSet(ConnectionSet);

#[wasm_bindgen]
#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PurpleSet(ConnectionSet);

#[wasm_bindgen]
#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GreenSet(ConnectionSet);

impl Deref for BlueSet {
    type Target = ConnectionSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for PurpleSet {
    type Target = ConnectionSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for GreenSet {
    type Target = ConnectionSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for YellowSet {
    type Target = ConnectionSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConnectionSet {
    theme: String,
    words: [String; 4],
}

impl From<YellowSet> for ConnectionSet {
    fn from(set: YellowSet) -> Self {
        set.0
    }
}

impl From<GreenSet> for ConnectionSet {
    fn from(set: GreenSet) -> Self {
        set.0
    }
}

impl From<PurpleSet> for ConnectionSet {
    fn from(set: PurpleSet) -> Self {
        set.0
    }
}

impl From<BlueSet> for ConnectionSet {
    fn from(set: BlueSet) -> Self {
        set.0
    }
}

#[wasm_bindgen]
impl ConnectionSet {
    pub fn theme(&self) -> String {
        self.theme.clone()
    }

    pub fn words_list(&self) -> Box<[String]> {
        Box::new(self.words.clone())
    }
}

impl ConnectionSet {
    fn new(theme: &str, words: [&str; 4]) -> Self {
        let words: [String; 4] = [
            words[0].into(),
            words[1].into(),
            words[2].into(),
            words[3].into(),
        ];
        Self {
            theme: theme.into(),
            words,
        }
    }

    pub fn theme_ref(&self) -> &str {
        &self.theme
    }

    pub fn words_list_ref(&self) -> [&str; 4] {
        self.words.each_ref().map(|str| str.as_str())
    }

    const fn empty_set() -> Self {
        Self {
            theme: String::new(),
            words: [const { String::new() }; 4],
        }
    }

    pub fn words(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.words[0], self.words[1], self.words[2], self.words[3]
        )
    }
}

impl std::ops::Index<usize> for ConnectionSet {
    type Output = str;

    fn index(&self, index: usize) -> &str {
        self.words[index].as_str()
    }
}

impl std::ops::Index<PuzzleRef> for ConnectionPuzzle {
    type Output = str;

    fn index(&self, index: PuzzleRef) -> &str {
        let set = self.by_color(index.color());
        index.word(set)
    }
}

/*
#[wasm_bindgen(module = "/index.js")]
extern "C" {
    pub type PuzzleArgs;
    #[wasm_bindgen(method, getter)]
    fn theme(this: &PuzzleArgs) -> String;

    #[wasm_bindgen(method, getter)]
    fn word(this: &PuzzleArgs, index: usize) -> String;

}

fn args_to_tuple(args: &PuzzleArgs) -> (String, [String; 4]) {
    let arr: [String; 4] = [args.word(0), args.word(1), args.word(2), args.word(3)];
    (args.theme(), arr)
}

fn args_as_ref((theme, words): &(String, [String; 4])) -> (&str, [&str; 4]) {
    let arr: [&str; 4] = [&words[0], &words[1], &words[2], &words[3]];
    (theme, arr)
}
*/

fn js_args(slice: &[String]) -> (&str, [&str; 4]) {
    assert_eq!(slice.len(), 5);
    (&slice[0], [&slice[1], &slice[2], &slice[3], &slice[4]])
}

#[wasm_bindgen]
impl ConnectionPuzzle {
    pub fn decode(code: &str) -> Result<Self, TranscodingError> {
        if code == "debug" {
            return Ok(Self::debug());
        } else if code == "default" {
            return Ok(Self::default());
        }
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

    pub fn from_js(
        yellow: Box<[String]>,
        blue: Box<[String]>,
        purple: Box<[String]>,
        green: Box<[String]>,
    ) -> Self {
        let yellow = js_args(&yellow);
        let blue = js_args(&blue);
        let purple = js_args(&purple);
        let green = js_args(&green);
        Self::new(yellow, blue, purple, green)
    }

    pub fn new_code(
        yellow: Box<[String]>,
        blue: Box<[String]>,
        purple: Box<[String]>,
        green: Box<[String]>,
    ) -> String {
        Self::from_js(yellow, blue, purple, green).encode()
    }

    pub fn encode(&self) -> String {
        let postcard_bytes: Vec<u8> = postcard::to_allocvec(&self).expect("error serializing");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&postcard_bytes).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        URL_SAFE.encode(&compressed_bytes)
    }

    pub fn yellow_owned(&self) -> ConnectionSet {
        self.yellow.clone().into()
    }

    pub fn blue_owned(&self) -> ConnectionSet {
        self.blue.clone().into()
    }
    pub fn green_owned(&self) -> ConnectionSet {
        self.green.clone().into()
    }
    pub fn purple_owned(&self) -> ConnectionSet {
        self.purple.clone().into()
    }
}
impl ConnectionPuzzle {
    pub fn yellow(&self) -> &ConnectionSet {
        &self.yellow
    }

    pub fn blue(&self) -> &ConnectionSet {
        &self.blue
    }
    pub fn green(&self) -> &ConnectionSet {
        &self.green
    }
    pub fn purple(&self) -> &ConnectionSet {
        &self.purple
    }

    fn debug() -> Self {
        let yellow = YellowSet(ConnectionSet::new("Yellow", ["y"; 4]));
        let blue = BlueSet(ConnectionSet::new("Blue", ["b"; 4]));
        let purple = PurpleSet(ConnectionSet::new("Purple", ["p"; 4]));
        let green = GreenSet(ConnectionSet::new("Green", ["g"; 4]));
        Self {
            yellow,
            blue,
            purple,
            green,
        }
    }

    pub const fn empty() -> Self {
        let yellow = YellowSet(ConnectionSet::empty_set());
        let blue = BlueSet(ConnectionSet::empty_set());
        let purple = PurpleSet(ConnectionSet::empty_set());
        let green = GreenSet(ConnectionSet::empty_set());

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
        let yellow = YellowSet(ConnectionSet::new(yellow.0, yellow.1));
        let blue = BlueSet(ConnectionSet::new(blue.0, blue.1));
        let purple = PurpleSet(ConnectionSet::new(purple.0, purple.1));
        let green = GreenSet(ConnectionSet::new(green.0, green.1));

        Self {
            yellow,
            blue,
            purple,
            green,
        }
    }

    pub fn theme(&self, reference: PuzzleRef) -> &str {
        let set = self.by_color(reference.color());
        &set[reference.word_index]
    }

    pub fn by_color(&self, color: Color) -> &ConnectionSet {
        match color {
            Color::Blue => &self.blue,
            Color::Green => &self.green,
            Color::Purple => &self.purple,
            Color::Yellow => &self.yellow,
        }
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
pub struct PuzzleRef {
    color: Color,
    word_index: usize,
}

impl PuzzleRef {
    const fn new(color: Color, word_index: usize) -> Self {
        Self { color, word_index }
    }

    pub const fn color(&self) -> Color {
        self.color
    }

    pub fn word<'a>(&self, set: &'a ConnectionSet) -> &'a str {
        set.words[self.word_index].as_ref()
    }

    pub const fn new_set() -> [Self; 16] {
        use Color::*;
        [
            Self::new(Yellow, 0),
            Self::new(Yellow, 1),
            Self::new(Yellow, 2),
            Self::new(Yellow, 3),
            Self::new(Blue, 0),
            Self::new(Blue, 1),
            Self::new(Blue, 2),
            Self::new(Blue, 3),
            Self::new(Green, 0),
            Self::new(Green, 1),
            Self::new(Green, 2),
            Self::new(Green, 3),
            Self::new(Purple, 0),
            Self::new(Purple, 1),
            Self::new(Purple, 2),
            Self::new(Purple, 3),
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PuzzleKey {
    pub color: Color,
    pub word_index: usize,
}

impl Default for PuzzleKey {
    fn default() -> Self {
        Self {
            color: Color::Yellow,
            word_index: 0,
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
