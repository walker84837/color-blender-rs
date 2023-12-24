use anyhow::Result;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct ColorBlender {
    start_color: String,
    end_color: String,
    precision: usize,
}

impl Default for ColorBlender {
    fn default() -> Self {
        ColorBlender {
            start_color: "#000000".to_string(),
            end_color: "#ffffff".to_string(),
            precision: 10,
        }
    }
}

impl ColorBlender {
    pub fn new(start_color: String, end_color: String, precision: usize) -> ColorBlender {
        ColorBlender {
            start_color,
            end_color,
            precision,
        }
    }

    pub fn blend_colors(&self) -> Vec<String> {
        let start = Self::parse_hex_color(self.start_color.clone());
        let end = Self::parse_hex_color(self.end_color.clone());

        let count = self.precision + 2;

        (0..count)
            .into_par_iter()
            .map(|i| {
                let step = i as f64 / (count - 1) as f64;
                let r = start.red + ((end.red - start.red) as f64 * step) as i32;
                let g = start.green + ((end.green - start.green) as f64 * step) as i32;
                let b = start.blue + ((end.blue - start.blue) as f64 * step) as i32;
                Color::new(r, g, b)
            })
            .map(ColorConverter::rgb_to_hex)
            .collect()
    }

    fn parse_hex_color(hex_color: String) -> Color {
        let hex_to_int = |s: &str| i32::from_str_radix(s, 16).unwrap_or(0);

        Color::new(
            hex_to_int(&hex_color[1..3]),
            hex_to_int(&hex_color[3..5]),
            hex_to_int(&hex_color[5..7]),
        )
    }
}

pub struct ColorConverter;

impl ColorConverter {
    pub fn rgb_to_hex(color: Color) -> String {
        format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
    }

    pub fn hex_to_rgb(hex_color: &str) -> Result<(u8, u8, u8)> {
        if !hex_color.starts_with('#') {
            return Err(anyhow::Error::msg("Error while parsing color"));
        }

        let hex_to_u8 = |s: &str| u8::from_str_radix(s, 16).unwrap_or(0);

        let r = hex_to_u8(&hex_color[1..3]);
        let g = hex_to_u8(&hex_color[3..5]);
        let b = hex_to_u8(&hex_color[5..7]);

        Ok((r, g, b))
    }
}

#[derive(PartialEq, Eq)]
pub struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl Color {
    fn new(red: i32, green: i32, blue: i32) -> Color {
        Color { red, green, blue }
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Color) -> Ordering {
        self.red
            .cmp(&other.red)
            .then(self.green.cmp(&other.green))
            .then(self.blue.cmp(&other.blue))
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Color) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_blending() {
        let blender = ColorBlender::new("#ff0000".to_string(), "#00ff00".to_string(), 5);
        let blended_colors = blender.blend_colors();
        assert_eq!(blended_colors.len(), 7);
    }

    #[test]
    fn hex_to_rgb_converting() {
        let rgb_result = ColorConverter::hex_to_rgb("#ff0000").unwrap();
        assert_eq!(rgb_result, (255, 0, 0));
    }
}
