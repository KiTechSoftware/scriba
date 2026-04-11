use crate::{Error, Result};

pub fn render(text: &str) -> Result<String> {
    render_with_font(text, "standard")
}

pub fn render_with_font(text: &str, font: &str) -> Result<String> {
    let figure = figlet_rs::FIGfont::from_standard_file(font)
        .map_err(|err| Error::Render(format!("failed to load figlet font '{font}': {err}")))?;

    let rendered = figure
        .convert(text)
        .ok_or_else(|| Error::Render("failed to render figlet text".to_string()))?;

    Ok(rendered.to_string())
}