use std::cmp::Ordering;

#[derive(Clone)]
pub struct ColorBlender {
    start_color: String,
    end_color: String,
    precision: usize,
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
            .map(|i| {
                let t = i as f64 / (count - 1) as f64;
                let r = start.red + ((end.red - start.red) as f64 * t) as i32;
                let g = start.green + ((end.green - start.green) as f64 * t) as i32;
                let b = start.blue + ((end.blue - start.blue) as f64 * t) as i32;
                Color::new(r, g, b)
            })
            .map(Self::color_to_hex)
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

    fn color_to_hex(color: Color) -> String {
        format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
    }
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

#[derive(/*Debug, */PartialEq, Eq/*, Clone*/)]
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
