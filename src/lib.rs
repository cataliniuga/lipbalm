pub mod colors;
pub mod styles;

use std::default;

use colors::Color;
use styles::Styles;

/// Style builde for terminal output. Call the methods to apply the desired styles and then call
/// `render` to apply the styles to the text.
/// Example:
/// ```
/// use lipbalm::Lipbalm;
/// use lipbalm::colors::Color;
/// use lipbalm::styles::Styles;
///
/// let lipbalm = Lipbalm::new()
///    .foreground(Color::Red)
///    .background(Color::Green)
///    .bold(true)
///    .underline(true)
///    .render("Hello, world!");
/// println!("{}", lipbalm);
/// ```
/// This will print "Hello, world!" in red text on a green background, with bold and underline
/// styles.
#[derive(Debug, Clone)]
pub struct Lipbalm {
    foreground: Option<Color>,
    background: Option<Color>,

    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    reverse: bool,
    hidden: bool,
    strikethrough: bool,

    link: Option<String>,
}

impl Lipbalm {
    pub fn new() -> Lipbalm {
        Lipbalm {
            foreground: None,
            background: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
            link: None,
        }
    }

    pub fn bold(mut self, yes: bool) -> Lipbalm {
        self.bold = yes;
        self
    }

    pub fn dim(mut self, yes: bool) -> Lipbalm {
        self.dim = yes;
        self
    }

    pub fn italic(mut self, yes: bool) -> Lipbalm {
        self.italic = yes;
        self
    }

    pub fn underline(mut self, yes: bool) -> Lipbalm {
        self.underline = yes;
        self
    }

    pub fn blink(mut self, yes: bool) -> Lipbalm {
        self.blink = yes;
        self
    }

    pub fn reverse(mut self, yes: bool) -> Lipbalm {
        self.reverse = yes;
        self
    }

    pub fn hidden(mut self, yes: bool) -> Lipbalm {
        self.hidden = yes;
        self
    }

    pub fn strikethrough(mut self, yes: bool) -> Lipbalm {
        self.strikethrough = yes;
        self
    }

    pub fn foreground(mut self, color: Color) -> Lipbalm {
        self.foreground = Some(color);
        self
    }

    pub fn link(mut self, link: &str) -> Lipbalm {
        self.link = Some(link.to_string());
        self
    }

    fn apply_foreground(&self) -> String {
        let value = self.foreground.unwrap();
        let ansi = value.to_ansi();
        match value {
            Color::Rgb(_, _, _) | Color::C256(_) | Color::Hex(_) => {
                format!("38;{}", ansi)
            }
            _ => ansi,
        }
    }

    pub fn background(mut self, color: Color) -> Lipbalm {
        self.background = Some(color);
        self
    }

    fn apply_background(&self) -> String {
        let value = self.background.unwrap_or(Color::Reset);
        let ansi = value.to_ansi();
        match value {
            Color::Reset => ansi,
            Color::Rgb(_, _, _) | Color::C256(_) | Color::Hex(_) => {
                format!("48;{}", ansi)
            }
            _ => {
                let nr = value.to_string().parse::<u8>().unwrap() + 10;
                nr.to_string()
            }
        }
    }

    /// Apply the styles to the text and return the result as a string.
    pub fn render(&self, text: &str) -> String {
        let mut styles: Vec<String> = Vec::new();

        if self.foreground.is_some() {
            styles.push(self.apply_foreground());
        }

        if self.background.is_some() {
            styles.push(self.apply_background());
        }

        if self.bold {
            styles.push(Styles::Bold.to_ansi());
        }

        if self.dim {
            styles.push(Styles::Dim.to_ansi());
        }

        if self.italic {
            styles.push(Styles::Italic.to_ansi());
        }

        if self.underline {
            styles.push(Styles::Underline.to_ansi());
        }

        if self.blink {
            styles.push(Styles::Blink.to_ansi());
        }

        if self.reverse {
            styles.push(Styles::Reverse.to_ansi());
        }

        if self.hidden {
            styles.push(Styles::Hidden.to_ansi());
        }

        if self.strikethrough {
            styles.push(Styles::Strikethrough.to_ansi());
        }

        let styles = styles.iter().map(|s| s.as_str()).filter(|s| !s.is_empty());
        let result = format!(
            "\x1b[{}m{}\x1b[0m",
            styles.collect::<Vec<&str>>().join(";"),
            text
        );

        if let Some(link) = &self.link {
            format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", link, result)
        } else {
            result
        }
    }
}

impl default::Default for Lipbalm {
    fn default() -> Lipbalm {
        Lipbalm::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_render() {
        let lipbalm = Lipbalm::new();
        let result = lipbalm.render("Hello, world!");
        assert_eq!(result, "\x1b[mHello, world!\x1b[0m");
    }

    #[test]
    fn with_styles() {
        let result = Lipbalm::new()
            .bold(true)
            .underline(true)
            .foreground(Color::Red)
            .background(Color::Green)
            .render("Hello, world!");
        assert_eq!(result, "\x1b[31;42;1;4mHello, world!\x1b[0m");
    }

    #[test]
    fn with_link() {
        let result = Lipbalm::new()
            .foreground(Color::Red)
            .link("https://example.com")
            .render("Hello, world!");
        assert_eq!(
            result,
            "\u{1b}]8;;https://example.com\u{1b}\\\u{1b}[31mHello, world!\u{1b}[0m\u{1b}]8;;\u{1b}\\"
        );
    }

    #[test]
    fn with_hex_color() {
        let result = Lipbalm::new()
            .foreground(Color::Hex("#ff0000"))
            .background(Color::Hex("#00ff00"))
            .render("Hello, world!");
        assert_eq!(
            result,
            "\x1b[38;2;255;0;0;48;2;0;255;0mHello, world!\x1b[0m"
        );
    }

    #[test]
    fn with_rgb_color() {
        let result = Lipbalm::new()
            .foreground(Color::Rgb(255, 0, 0))
            .background(Color::Rgb(0, 255, 0))
            .render("Hello, world!");
        assert_eq!(
            result,
            "\x1b[38;2;255;0;0;48;2;0;255;0mHello, world!\x1b[0m"
        );
    }

    #[test]
    fn with_c256_color() {
        let result = Lipbalm::new()
            .foreground(Color::C256(1))
            .background(Color::C256(2))
            .render("Hello, world!");
        assert_eq!(result, "\x1b[38;5;1;48;5;2mHello, world!\x1b[0m");
    }

    #[test]
    fn with_reset_color() {
        let result = Lipbalm::new()
            .foreground(Color::Red)
            .foreground(Color::Reset)
            .render("Hello, world!");
        assert_eq!(result, "\x1b[0mHello, world!\x1b[0m");
    }

    #[test]
    fn with_unset_style() {
        let result = Lipbalm::new()
            .bold(true)
            .underline(true)
            .bold(false)
            .underline(false)
            .render("Hello, world!");
        assert_eq!(result, "\x1b[mHello, world!\x1b[0m");
    }
}
