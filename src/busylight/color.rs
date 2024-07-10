extern crate hex_color;

use hex_color::HexColor;

pub enum Color {
    // Primary colors
    ///
    Red,
    Green,
    Blue,

    /// Yellow is Red+Green
    Yellow,

    /// Cyan is Green+Blue
    Cyan,

    /// Magenta is Blue+Red
    Magenta,

    /// Custom RGB value
    Custom {
        red: u8,
        green: u8,
        blue: u8,
    }
}

impl Color {

    pub fn parse(str: &String) -> Option<Color> {
        Self::parse_literal(str).or(Self::match_hex(str))
    }

    fn parse_literal(str: &String) -> Option<Color> {
        match str.to_lowercase().as_ref() {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            "yellow" => Some(Color::Yellow),
            "cyan" => Some(Color::Cyan),
            "magenta" => Some(Color::Magenta),
            _ => None
        }
    }

    fn match_hex(str: &String) -> Option<Color> {
        match HexColor::parse(str.as_ref()) {
            Ok(color) => Some(Color::Custom {red: color.r, green: color.g, blue: color.b}),
            Err(_) => None,
        }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Red => (255,0,0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Cyan => (0, 255, 255),
            Color::Magenta => (255, 0, 255),
            Color::Custom { red, green, blue } => (*red, *green, *blue),
        }
    }

}

