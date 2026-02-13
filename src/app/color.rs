use color::{Lab, OpaqueColor};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

type Channel = u8;

pub fn saturating_add(channel: Channel, rhs: i32) -> Channel {
    (channel as i32 + rhs).clamp(0, u8::MAX as i32) as u8
}

impl Color {
    pub fn default() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn try_from_hex_str(hex: &str) -> Result<Self, ()> {
        if hex.len() != 6 {
            return Err(());
        }

        let Ok(r) = u8::from_str_radix(&hex[0..2], 16) else {
            return Err(());
        };
        let Ok(g) = u8::from_str_radix(&hex[2..4], 16) else {
            return Err(());
        };
        let Ok(b) = u8::from_str_radix(&hex[4..6], 16) else {
            return Err(());
        };

        Ok(Self::new(r, g, b))
    }

    // TODO: change to color's functions here
    pub fn adjust_red(&mut self, amount: i32) {
        self.r = saturating_add(self.r, amount)
    }
    pub fn adjust_green(&mut self, amount: i32) {
        self.g = saturating_add(self.g, amount)
    }
    pub fn adjust_blue(&mut self, amount: i32) {
        self.b = saturating_add(self.b, amount)
    }
    pub fn adjust_lightness(&mut self, amount: i32) {
        let (r, g, b) = self.rgb();
        let color: OpaqueColor<Lab> = OpaqueColor::from_rgb8(r, g, b).convert();

        let color = color.map_lightness(|l| l + amount as f32 / 256.).to_rgba8();
        *self = Self::new(color.r, color.g, color.b)
    }
    pub fn adjust_hue(&mut self, amount: i32) {
        let (r, g, b) = self.rgb();
        let color: OpaqueColor<Lab> = OpaqueColor::from_rgb8(r, g, b).convert();

        let color = color
            .map_hue(|h| (h + amount as f32 / 256. * 360.) % 360.)
            .to_rgba8();
        *self = Self::new(color.r, color.g, color.b)
    }
}
