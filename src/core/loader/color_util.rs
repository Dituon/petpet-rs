use skia_safe::Color;
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;

pub fn parse_color(s: &str) -> Result<Color, Error> {
    let c = color_art::Color::from_hex(s)
        .map_err(|e| TemplateError(format!("parse Color error: {}", e)))?;
    Ok(Color::from_argb(
        (255.0 * c.alpha()) as u8,
        c.red(), c.green(), c.blue()
    ))
}