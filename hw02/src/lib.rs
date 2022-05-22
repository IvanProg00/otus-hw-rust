pub struct SmartSocket {
    description: String,
}

impl SmartSocket {
    pub fn new(id: &str) -> Self {
        SmartSocket {
            description: format!("Hello, I'm Smart Socket, with id: {}", id),
        }
    }

    pub fn about(&self) -> &str {
        self.description.as_str()
    }

    pub fn turn_on() {
        todo!()
    }

    pub fn turn_off() {
        todo!()
    }

    pub fn get_power_info() {
        todo!()
    }
}

pub struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            temperature: Self::set_temperature(),
        }
    }

    fn set_temperature() -> f32 {
        todo!()
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

impl Default for Thermometer {
    fn default() -> Self {
        Self::new()
    }
}
