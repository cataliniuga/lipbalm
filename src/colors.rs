use std::fmt;

#[derive(Clone, Copy)]
#[repr(u8)]
/// ANSI color codes. Includes 16 basic colors, 256 colors, RGB and HEX.
/// Note: Expected format for HEX is `#RRGGBB`.
pub enum Color {
    Reset = 0,
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8),
    C256(u8),
    Hex(&'static str),
}

impl Color {
    pub(crate) fn to_ansi(self) -> String {
        match self {
            Color::Reset => "0".to_string(),
            Color::Black => "30".to_string(),
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::BrightBlack => "90".to_string(),
            Color::BrightRed => "91".to_string(),
            Color::BrightGreen => "92".to_string(),
            Color::BrightYellow => "93".to_string(),
            Color::BrightBlue => "94".to_string(),
            Color::BrightMagenta => "95".to_string(),
            Color::BrightCyan => "96".to_string(),
            Color::BrightWhite => "97".to_string(),
            Color::C256(c) => format!("5;{}", c),
            Color::Rgb(r, g, b) => format!("2;{};{};{}", r, g, b),
            Color::Hex(s) => {
                let (r, g, b) = hex_to_rgb(s.to_string());
                format!("2;{};{};{}", r, g, b)
            }
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn hex_to_rgb(hex: String) -> (u8, u8, u8) {
    let hex = u32::from_str_radix(&hex[1..], 16).unwrap_or(0);
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FF0000".to_string()), (255, 0, 0));
        assert_eq!(hex_to_rgb("#00FF00".to_string()), (0, 255, 0));
        assert_eq!(hex_to_rgb("#0000FF".to_string()), (0, 0, 255));
    }
}
