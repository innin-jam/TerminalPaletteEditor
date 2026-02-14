use color::{Oklch, OpaqueColor, Srgb};
use eyre::{Result, eyre};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    color: OpaqueColor<Oklch>,
}

impl Color {
    pub fn default() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let rgb = OpaqueColor::from_rgb8(r, g, b);
        Self {
            color: rgb.convert(),
        }
    }

    pub fn hex(&self) -> String {
        let [r, g, b, _] = self.color.convert::<Srgb>().to_rgba8().to_u8_array();
        format!("{:02x}{:02x}{:02x}", r, g, b)
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        let [r, g, b, _] = self.color.convert::<Srgb>().to_rgba8().to_u8_array();
        (r, g, b)
    }

    pub fn try_from_hex_str(hex: &str) -> Result<Self> {
        if hex.len() != 6 {
            return Err(eyre!("Failed to parse color from: '{hex}'"));
        }
        let Ok(r) = u8::from_str_radix(&hex[0..2], 16) else {
            return Err(eyre!("Failed to parse color from: '{hex}'"));
        };
        let Ok(g) = u8::from_str_radix(&hex[2..4], 16) else {
            return Err(eyre!("Failed to parse color from: '{hex}'"));
        };
        let Ok(b) = u8::from_str_radix(&hex[4..6], 16) else {
            return Err(eyre!("Failed to parse color from: '{hex}'"));
        };
        Ok(Self::new(r, g, b))
    }

    pub fn adjust_lightness(&mut self, amount: f32) {
        let lightness = &mut self.color.components[0];
        *lightness = (*lightness + amount).clamp(0., 1.);
    }
    pub fn adjust_chroma(&mut self, amount: f32) {
        let chroma = &mut self.color.components[1];
        *chroma = (*chroma + amount).clamp(0., 1.);
    }
    pub fn adjust_hue(&mut self, amount: f32) {
        let hue = &mut self.color.components[2];
        *hue = (*hue + amount * 360.) % 360.;
    }
}
