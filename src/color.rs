// use this module to automate the creation of CSS files.

use crate::utils::{into_v32, from_v32};
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Color {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

/// It returns the default colors and their names used by the engine.
pub fn colors() -> Vec<Color> {
    COLORS.to_vec()
}

impl Color {

    pub fn new(name: &str, r: u8, g: u8, b: u8) -> Self {
        Color {
            name: name.to_string(), r, g, b
        }
    }

    pub fn complement(&self) -> Self {
        Color {
            name: self.name.clone(),
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b
        }
    }

    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_hex(&self) -> String {
        format!(
            "#{}{}{}",
            from_v32(&into_v32(&format!("{:#04x}", self.r))[2..4]),
            from_v32(&into_v32(&format!("{:#04x}", self.g))[2..4]),
            from_v32(&into_v32(&format!("{:#04x}", self.b))[2..4])
        )
    }

}

lazy_static! {
    pub static ref COLORS: Vec<Color> = vec![
        Color::new("black", 0, 0, 0),
        Color::new("dark", 64, 64, 64),
        Color::new("gray", 128, 128, 128),
        Color::new("lightgray", 192, 192, 192),
        Color::new("white", 255, 255, 255),
        Color::new("red", 192, 32, 32),
        Color::new("green", 32, 192, 32),
        Color::new("slateblue", 64, 64, 192),
        Color::new("blue", 32, 32, 192),
        Color::new("aqua", 64, 192, 255),
        Color::new("emerald", 64, 192, 64),
        Color::new("turquoise", 64, 255, 192),
        Color::new("seagreen", 32, 192, 192),
        Color::new("violet", 192, 64, 255),
        Color::new("pink", 255, 64, 192),
        Color::new("grassgreen", 192, 255, 64),
        Color::new("gold", 255, 192, 64),
        Color::new("brown", 192, 128, 32),
    ];

    pub static ref COLOR_NAMES: Vec<Vec<u32>> = COLORS.iter().map(
        |color| into_v32(&color.name)
    ).collect();
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn hex_color_test() {
        assert_eq!(Color::new("", 0, 0, 0).to_hex(), String::from("#000000"));
        assert_eq!(Color::new("", 0, 255, 0).to_hex(), String::from("#00ff00"));
        assert_eq!(Color::new("", 64, 255, 64).to_hex(), String::from("#40ff40"));
    }

}