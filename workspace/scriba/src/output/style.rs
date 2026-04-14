//! Text styling for rich output formatting.
//!
//! Apply semantic styles to text content: bold, italic, underline, strikethrough, and dim.
//! Styles are rendered format-appropriately (ANSI codes for Text, Markdown syntax, etc.).

use serde::{Deserialize, Serialize};

/// Text styling variants.
///
/// Each style is rendered format-appropriately:
/// - **Text**: ANSI escape codes
/// - **Markdown**: Markdown syntax (`**bold**`, `*italic*`, etc.)
/// - **Other formats**: Style info stored but rendering may be limited
///
/// # Examples
///
/// ```
/// use scriba::TextStyle;
///
/// let bold = TextStyle::Bold;
/// let italic = TextStyle::Italic;
/// let combined = TextStyle::BoldItalic;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TextStyle {
    /// Bold/strong emphasis.
    Bold,
    /// Italic/emphasis.
    Italic,
    /// Bold and italic combined.
    BoldItalic,
    /// Underlined text.
    Underline,
    /// Strikethrough text.
    Strikethrough,
    /// Dimmed/faded text.
    Dim,
}

impl TextStyle {
    /// Apply ANSI styling to text for terminal display.
    pub fn apply_ansi(self, text: &str) -> String {
        match self {
            TextStyle::Bold => format!("\x1b[1m{}\x1b[0m", text),
            TextStyle::Italic => format!("\x1b[3m{}\x1b[0m", text),
            TextStyle::BoldItalic => format!("\x1b[1;3m{}\x1b[0m", text),
            TextStyle::Underline => format!("\x1b[4m{}\x1b[0m", text),
            TextStyle::Strikethrough => format!("\x1b[9m{}\x1b[0m", text),
            TextStyle::Dim => format!("\x1b[2m{}\x1b[0m", text),
        }
    }

    /// Apply Markdown syntax for styled text.
    pub fn apply_markdown(self, text: &str) -> String {
        match self {
            TextStyle::Bold => format!("**{}**", text),
            TextStyle::Italic => format!("*{}*", text),
            TextStyle::BoldItalic => format!("***{}***", text),
            TextStyle::Underline => format!("<u>{}</u>", text),
            TextStyle::Strikethrough => format!("~~{}~~", text),
            TextStyle::Dim => text.to_string(), // Markdown doesn't have dim
        }
    }

    /// Returns `true` if this style includes bold.
    pub fn is_bold(self) -> bool {
        matches!(self, TextStyle::Bold | TextStyle::BoldItalic)
    }

    /// Returns `true` if this style includes italic.
    pub fn is_italic(self) -> bool {
        matches!(self, TextStyle::Italic | TextStyle::BoldItalic)
    }
}

/// Styled text — text content with one or more applied styles.
///
/// # Examples
///
/// ```
/// use scriba::{TextStyle, Styled};
///
/// let styled = Styled::new("Important", TextStyle::Bold);
/// assert_eq!(styled.text, "Important");
/// assert_eq!(styled.style, TextStyle::Bold);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Styled {
    /// The text content.
    pub text: String,
    /// The applied style.
    pub style: TextStyle,
}

impl Styled {
    /// Create new styled text.
    pub fn new(text: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    /// Render styled text with ANSI codes (for Text format).
    pub fn render_ansi(&self) -> String {
        self.style.apply_ansi(&self.text)
    }

    /// Render styled text with Markdown syntax.
    pub fn render_markdown(&self) -> String {
        self.style.apply_markdown(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_style_apply_ansi() {
        let bold = TextStyle::Bold.apply_ansi("test");
        assert!(bold.contains("\x1b[1m"));
        assert!(bold.contains("\x1b[0m"));
        assert!(bold.contains("test"));
    }

    #[test]
    fn text_style_apply_markdown() {
        assert_eq!(TextStyle::Bold.apply_markdown("test"), "**test**");
        assert_eq!(TextStyle::Italic.apply_markdown("test"), "*test*");
        assert_eq!(TextStyle::BoldItalic.apply_markdown("test"), "***test***");
        assert_eq!(
            TextStyle::Strikethrough.apply_markdown("test"),
            "~~test~~"
        );
    }

    #[test]
    fn text_style_properties() {
        assert!(TextStyle::Bold.is_bold());
        assert!(!TextStyle::Bold.is_italic());
        assert!(TextStyle::BoldItalic.is_bold());
        assert!(TextStyle::BoldItalic.is_italic());
    }

    #[test]
    fn styled_new_and_render() {
        let styled = Styled::new("hello", TextStyle::Bold);
        assert_eq!(styled.text, "hello");
        assert_eq!(styled.style, TextStyle::Bold);

        let md = styled.render_markdown();
        assert_eq!(md, "**hello**");
    }

    #[test]
    fn text_style_all_ansi_codes() {
        assert!(TextStyle::Italic.apply_ansi("t").contains("\x1b[3m"));
        assert!(TextStyle::BoldItalic.apply_ansi("t").contains("\x1b[1;3m"));
        assert!(TextStyle::Underline.apply_ansi("t").contains("\x1b[4m"));
        assert!(TextStyle::Strikethrough.apply_ansi("t").contains("\x1b[9m"));
        assert!(TextStyle::Dim.apply_ansi("t").contains("\x1b[2m"));
    }

    #[test]
    fn styled_render_ansi_and_markdown() {
        let s = Styled::new("test", TextStyle::Underline);
        assert!(s.render_ansi().contains("\x1b[4m"));
        assert_eq!(s.render_markdown(), "<u>test</u>");
    }

    #[test]
    fn styled_serde_round_trip() {
        let original = Styled::new("important", TextStyle::Bold);
        let json = serde_json::to_string(&original).unwrap();
        let restored: Styled = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn text_style_serde_round_trip() {
        for style in [
            TextStyle::Bold,
            TextStyle::Italic,
            TextStyle::BoldItalic,
            TextStyle::Underline,
            TextStyle::Strikethrough,
            TextStyle::Dim,
        ] {
            let json = serde_json::to_string(&style).unwrap();
            let restored: TextStyle = serde_json::from_str(&json).unwrap();
            assert_eq!(style, restored);
        }
    }
}
