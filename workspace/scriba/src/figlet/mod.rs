//! ASCII art rendering using figlet fonts.
//!
//! Requires the `figlet` feature.
//!
//! Supports built-in fonts (standard, small, big, slant, etc.) and custom font files.

use crate::{Error, Result};
use figlet_rs::{FIGlet, Toilet};

/// Render text as ASCII art using the standard figlet font.
///
/// # Example
///
/// ```ignore
/// let ascii = scriba::figlet::render("Hello")?;
/// println!("{}", ascii);
/// ```
pub fn render(text: &str) -> Result<String> {
    render_with_font(text, "standard")
}

/// Render text as ASCII art using a specified font.
///
/// # Built-in Fonts
///
/// - `standard` — Default monospace font
/// - `small` — Smaller variant
/// - `big` — Larger variant
/// - `slant` — Slanted/italic appearance
/// - `smblock`, `mono12`, `future`, `wideterm`, `mono9` — Additional fonts
///
/// # Custom Fonts
///
/// Provide a path to a custom `.flf` or `.tlf` font file:
/// ```ignore
/// let ascii = scriba::figlet::render_with_font("Text", "/path/to/font.flf")?;
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Font is unknown or unavailable
/// - Text cannot be rendered with the font
///
/// # Example
///
/// ```ignore
/// let ascii = scriba::figlet::render_with_font("Big", "big")?;
/// println!("{}", ascii);
/// ```
pub fn render_with_font(text: &str, font: &str) -> Result<String> {
    let normalized = normalize_font_name(font);

    macro_rules! render_font {
        ($font_loader:expr) => {{
            let font = $font_loader.map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }};
    }

    match normalized.as_str() {
        "standard" => render_font!(FIGlet::standard()),
        "small" => render_font!(FIGlet::small()),
        "big" => render_font!(FIGlet::big()),
        "slant" => render_font!(FIGlet::slant()),
        "smblock" => render_font!(Toilet::smblock()),
        "mono12" => render_font!(Toilet::mono12()),
        "future" => render_font!(Toilet::future()),
        "wideterm" => render_font!(Toilet::wideterm()),
        "mono9" => render_font!(Toilet::mono9()),
        
        _ if looks_like_file(font) => {
            if font.ends_with(".tlf") {
                render_font!(Toilet::from_file(font))
            } else {
                render_font!(FIGlet::from_file(font))
            }
        }

        other => Err(Error::Render(format!(
            "unknown figlet font '{other}'. built-in fonts: standard, small, big, slant, smblock, mono12, future, wideterm, mono9"
        ))),
    }
}

fn normalize_font_name(font: &str) -> String {
    font.trim().to_ascii_lowercase()
}

fn looks_like_file(font: &str) -> bool {
    font.contains('/') || font.contains('\\') || font.ends_with(".flf") || font.ends_with(".tlf")
}

fn render_failed() -> Error {
    Error::Render("failed to render figlet text".to_string())
}

fn map_figlet_error(err: impl std::fmt::Display) -> Error {
    Error::Render(err.to_string())
}
