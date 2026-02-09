use color::{Oklab, OpaqueColor};

// TODO: Change to Oklab colorspace as the main representation of colors

#[derive(Debug, Clone, Copy)]
pub struct Color {
    color: OpaqueColor<Oklab>,
}

// TODO: remove channel, instead use the color's stuff

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

    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
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
    pub fn add_red(&mut self, r: i32) {
        self.r = saturating_add(self.r, r)
    }
    pub fn add_green(&mut self, g: i32) {
        self.g = saturating_add(self.g, g)
    }
    pub fn add_blue(&mut self, b: i32) {
        self.b = saturating_add(self.b, b)
    }
}
