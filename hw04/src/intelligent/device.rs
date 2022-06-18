pub struct Device {
    pub name: String,
}

impl Device {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
