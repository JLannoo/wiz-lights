use crate::Light::LightColor;

pub struct MessageConstructor;

impl MessageConstructor {
    pub fn get_system_config() -> String {
        r#"{"method":"getSystemConfig","params":{}}"#.to_string()
    }

    pub fn get_pilot_state() -> String {
        r#"{"method":"getPilot","params":{}}"#.to_string()
    }

    pub fn set_pilot_state(color: LightColor, dimming: u8, state: bool) -> String {
        let state = if state { "on" } else { "off" };

        let params = match color {
            LightColor::RGB(r,g,b) => format!(r#"{{ "r": {}, "g": {}, "b": {}, "dimming": {}, "state": "{}" }}"#, r, g, b, dimming, state),
            LightColor::Temperature(temp) => format!(r#"{{ "temp": {}, "dimming": {}, "state": "{}" }}"#, temp, dimming, state),
        };

        format!(r#"{{"method":"setPilot","params":{}}}"#, params)
    }
}