use crate::error::{ColorError, ColorResult};
use std::{cmp::Ordering, ops::RangeInclusive};

/// A color with red, green, and blue components normalized to [0.0, 1.0].
#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

const NORMALIZED_RANGE: RangeInclusive<f32> = 0.0..=1.0;

impl Color {
    /// Create a new Color from normalized f32 components.
    pub fn new(red: f32, green: f32, blue: f32) -> Option<Color> {
        if NORMALIZED_RANGE.contains(&red)
            && NORMALIZED_RANGE.contains(&green)
            && NORMALIZED_RANGE.contains(&blue)
        {
            return Some(Color { red, green, blue });
        }
        None
    }

    /// Convert this Color to a hexadecimal `#rrggbb` string.
    pub fn to_hex(&self) -> String {
        let r = (self.red.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (self.green.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (self.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Parse a hex string of the form `#rrggbb` into a `Color`.
    ///
    /// Returns an error if the string is not exactly 7 characters long or
    /// does not start with `#`, or if any pair of hex digits fails to parse.
    pub fn from_hex(hex: &str) -> ColorResult<Color> {
        if !hex.starts_with('#') || hex.len() != 7 {
            return Err(ColorError::WrongFormat(
                "Hex color must be in format `#rrggbb`".into(),
            ));
        }
        let r = Self::hex_to_u8(&hex[1..3])?;
        let g = Self::hex_to_u8(&hex[3..5])?;
        let b = Self::hex_to_u8(&hex[5..7])?;
        Ok(Color::from((r, g, b)))
    }

    /// Returns an array of the three non-normalized RGB components, as an array.
    pub fn to_u8_array(&self) -> [u8; 3] {
        [
            (self.red.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.green.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    /// Returns an array of the three normalized RGB components, as an array.
    pub fn to_array(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    fn hex_to_u8(s: &str) -> ColorResult<u8> {
        u8::from_str_radix(s, 16).map_err(|e| ColorError::ParseError(e))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl Eq for Color {}

impl Ord for Color {
    fn cmp(&self, other: &Color) -> Ordering {
        self.red
            .partial_cmp(&other.red)
            .unwrap()
            .then(self.green.partial_cmp(&other.green).unwrap())
            .then(self.blue.partial_cmp(&other.blue).unwrap())
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Color) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Trait for blending two values of the same type into a sequence of intermediate steps.
pub trait Blend: Sized + PartialEq {
    /// Blend `self` and `other` into `midpoints` intermediate colors (in addition to the start and end).
    ///
    /// For example, if `midpoints = 0`, this returns a Vec of length 2: `[self, other]`.
    /// If `midpoints = 1`, this returns `[self, midpoint, other]`, and so on.
    fn blend(self, other: Self, midpoints: usize) -> Vec<Self>;

    /// Same as [`blend`], but deduplicates the resulting vector of colors (in case identical colors appear).
    /// Because this method *deduplicates* the colors, the resulting vector is not guaranteed to be of size
    /// `midpoints`.
    fn blend_unique(self, other: Self, midpoints: usize) -> Vec<Self> {
        let mut all = self.blend(other, midpoints);
        all.dedup();
        all
    }
}

impl Blend for Color {
    fn blend(self, other: Color, midpoints: usize) -> Vec<Color> {
        // number of total steps = midpoints + 2 (including start and end)
        let total_steps = midpoints + 2;
        let mut result = Vec::with_capacity(total_steps);

        // TODO: should this be SIMD or add it but keep impl behind a feature?
        for i in 0..total_steps {
            let t = i as f32 / (total_steps - 1) as f32;
            let r = self.red + (other.red - self.red) * t;
            let g = self.green + (other.green - self.green) * t;
            let b = self.blue + (other.blue - self.blue) * t;
            result.push(Color::new(r / 255.0, g / 255.0, b / 255.0).unwrap());
        }

        result
    }
}

/// Convert a `(u8, u8, u8)` tuple (0..255 each) into a `Color` (0.0..1.0 each).
impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Color {
        Color {
            red: rgb.0 as f32 / 255.0,
            green: rgb.1 as f32 / 255.0,
            blue: rgb.2 as f32 / 255.0,
        }
    }
}

/// Convert a `(f32, f32, f32)` tuple (0.0..=1.0) into a `Color`.
impl TryFrom<(f32, f32, f32)> for Color {
    type Error = crate::error::ColorError;

    fn try_from(components: (f32, f32, f32)) -> Result<Color, Self::Error> {
        if NORMALIZED_RANGE.contains(&components.0)
            || NORMALIZED_RANGE.contains(&components.1)
            || NORMALIZED_RANGE.contains(&components.2)
        {
            return Ok(Color {
                red: components.0,
                green: components.1,
                blue: components.2,
            });
        }

        Err(ColorError::WrongFormat(
            "Normalized colors should be from 0 to 1".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_tuple() {
        let c = Color::from((128u8, 64u8, 255u8));
        assert!((c.red - 128.0 / 255.0).abs() < f32::EPSILON);
        assert!((c.green - 64.0 / 255.0).abs() < f32::EPSILON);
        assert!((c.blue - 255.0 / 255.0).abs() < f32::EPSILON);
    }

    #[test]
    fn from_f32_tuple() {
        let c = Color::try_from((0.1f32, 0.2f32, 0.3f32)).unwrap();
        assert!((c.red - 0.1).abs() < f32::EPSILON);
        assert!((c.green - 0.2).abs() < f32::EPSILON);
        assert!((c.blue - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn to_hex_and_from_hex_roundtrip() {
        let original = Color::from((200u8, 100u8, 50u8));
        let hex = original.to_hex();
        assert_eq!(hex, "#c86432");
        let parsed = Color::from_hex(&hex).unwrap();
        // /255.0 exactly matches the original normalized components
        assert!((parsed.red - original.red).abs() < f32::EPSILON);
        assert!((parsed.green - original.green).abs() < f32::EPSILON);
        assert!((parsed.blue - original.blue).abs() < f32::EPSILON);
    }

    #[test]
    fn invalid_hex_rejected() {
        assert!(Color::from_hex("ff0000").is_err()); // missing '#'
        assert!(Color::from_hex("#ff").is_err()); // too short
        assert!(Color::from_hex("#gg0000").is_err()); // invalid hex digits
    }

    #[test]
    fn color_blend_length() {
        let start = Color::from((255u8, 0u8, 0u8)); // red
        let end = Color::from((0u8, 255u8, 0u8)); // green
        let blended = start.blend(end, 5);
        assert_eq!(blended.len(), 7);
        // first should be exactly start, last exactly end
        assert_eq!(blended.first().unwrap(), &start);
        assert_eq!(blended.last().unwrap(), &end);
    }

    #[test]
    fn color_blend_unique() {
        let a = Color::from((10u8, 20u8, 30u8));
        let b = a; // same color
                   // with zero midpoints, blend should return [a, b], but blend_unique should dedupe to [a].
        let blended = a.blend_unique(b, 0);
        assert_eq!(blended.len(), 1);
        assert_eq!(blended[0], a);
    }

    #[test]
    fn ordering_and_equality() {
        let c1 = Color::from((100u8, 150u8, 200u8));
        let c2 = Color::from((100u8, 150u8, 200u8));
        let c3 = Color::from((100u8, 150u8, 201u8));
        assert_eq!(c1, c2);
        assert!(c1 < c3);
    }
}
