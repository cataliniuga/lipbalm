/// ANSI styles codes. Includes basic text styles.
pub enum Styles {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
}

impl Styles {
    pub fn to_ansi(&self) -> String {
        match self {
            Styles::Reset => "0".to_string(),
            Styles::Bold => "1".to_string(),
            Styles::Dim => "2".to_string(),
            Styles::Italic => "3".to_string(),
            Styles::Underline => "4".to_string(),
            Styles::Blink => "5".to_string(),
            Styles::Reverse => "7".to_string(),
            Styles::Hidden => "8".to_string(),
            Styles::Strikethrough => "9".to_string(),
        }
    }
}
