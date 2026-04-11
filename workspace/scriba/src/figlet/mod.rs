use crate::{Error, Result};
use figlet_rs::{FIGlet, Toilet};

pub fn render(text: &str) -> Result<String> {
    render_with_font(text, "standard")
}

pub fn render_with_font(text: &str, font: &str) -> Result<String> {
    let normalized = normalize_font_name(font);

    match normalized.as_str() {
        "standard" => {
            let font = FIGlet::standard().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "small" => {
            let font = FIGlet::small().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "big" => {
            let font = FIGlet::big().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "slant" => {
            let font = FIGlet::slant().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }

        "smblock" => {
            let font = Toilet::smblock().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "mono12" => {
            let font = Toilet::mono12().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "future" => {
            let font = Toilet::future().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "wideterm" => {
            let font = Toilet::wideterm().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }
        "mono9" => {
            let font = Toilet::mono9().map_err(map_figlet_error)?;
            let figure = font.convert(text).ok_or_else(render_failed)?;
            Ok(figure.to_string())
        }

        _ if looks_like_file(font) => {
            if font.ends_with(".tlf") {
                let font = Toilet::from_file(font).map_err(map_figlet_error)?;
                let figure = font.convert(text).ok_or_else(render_failed)?;
                Ok(figure.to_string())
            } else {
                let font = FIGlet::from_file(font).map_err(map_figlet_error)?;
                let figure = font.convert(text).ok_or_else(render_failed)?;
                Ok(figure.to_string())
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
