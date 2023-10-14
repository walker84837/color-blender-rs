use std::cmp::Ordering;

pub struct ColorBlender;

impl ColorBlender {
    pub fn new() -> ColorBlender {
        ColorBlender
    }

    pub fn blend_colors(
        &self,
        start_color: &str,
        end_color: &str,
        precision: usize,
    ) -> Vec<String> {
        let start = Self::parse_hex_color(start_color);
        let end = Self::parse_hex_color(end_color);

        let count = precision + 2;

        let mut palette = Vec::with_capacity(count);
        palette.push(start.clone());
        for i in 1..count - 1 {
            let r = start.red + ((end.red - start.red) * i as i32) / (count as i32 - 1);
            let g = start.green + ((end.green - start.green) * i as i32) / (count as i32 - 1);
            let b = start.blue + ((end.blue - start.blue) * i as i32) / (count as i32 - 1);
            palette.push(Color::new(r, g, b));
        }
        palette.push(end);

        palette
            .into_iter()
            .map(|color| Self::color_to_hex(color))
            .collect()
    }

    fn parse_hex_color(hex_color: &str) -> Color {
        let r = i32::from_str_radix(&hex_color[1..3], 16).unwrap();
        let g = i32::from_str_radix(&hex_color[3..5], 16).unwrap();
        let b = i32::from_str_radix(&hex_color[5..7], 16).unwrap();
        Color::new(r, g, b)
    }

    fn color_to_hex(color: Color) -> String {
        format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Color {
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
