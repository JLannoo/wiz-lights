use crate::Messenger::MessageConstructor;

pub enum LightColor {
    RGB(u8, u8, u8),
    Temperature(u16),
}

pub struct Light {
    pub ip: String,
    pub port: u16,
    pub mac: String,
    pub color: LightColor,
    pub dimming: u8,
    pub state: bool,
}

impl Light {
    pub fn new(ip: String, port: u16, mac: String) -> Self {
        Self {
            ip,
            port,
            mac,
            color: LightColor::RGB(0, 0, 0),
            dimming: 0,
            state: false,
        }
    }

    pub fn set_color(&mut self, color: LightColor) {
        self.color = color;
    }

    pub fn set_dimming(&mut self, dimming: u8) {
        self.dimming = dimming;
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }
}

impl std::fmt::Debug for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{} {} {} {}%", self.ip, self.port, self.mac, self.color, self.dimming)
    }
}

impl std::fmt::Display for LightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LightColor::RGB(r, g, b) => write!(f, "RGB({}, {}, {})", r, g, b),
            LightColor::Temperature(k) => write!(f, "Temperature({}K)", k),
        }
    }
}