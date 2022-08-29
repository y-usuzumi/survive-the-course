use self::Color::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    LightRed,
    LightGreen,
    LightBlue,
}

lazy_static! {
    static ref COLOR_MAP: HashMap<Color, (i32, i32)> = HashMap::from([
        (Black, (0, 30)),
        (Red, (0, 31)),
        (Green, (0, 32)),
        (Blue, (0, 34)),
        (LightRed, (1, 31)),
        (LightGreen, (1, 32)),
        (LightBlue, (1, 34)),
    ]);
}

pub fn colorize(s: &str, color: Color) -> String {
    let (idx0, idx1) = COLOR_MAP[&color];
    format!("\x1b[{};{}m{}\x1b[0m", idx0, idx1, s)
}
