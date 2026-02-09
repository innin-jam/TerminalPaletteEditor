#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: Channel,
    pub g: Channel,
    pub b: Channel,
}

type Channel = u8;

pub fn saturating_add(channel: Channel, rhs: i16) -> Channel {
    (channel as i16 + rhs).clamp(0, u8::MAX as i16) as u8
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

    pub fn add_red(&mut self, r: i16) {
        self.r = saturating_add(self.r, r)
    }
    pub fn add_green(&mut self, g: i16) {
        self.g = saturating_add(self.g, g)
    }
    pub fn add_blue(&mut self, b: i16) {
        self.b = saturating_add(self.b, b)
    }
}
