use crate::intelligent::device;

pub struct Room {
    pub name: String,
    pub devices: Vec<device::Device>,
}

impl Room {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn push_device(&mut self, name: String) -> Result<(), String> {
        for d in self.devices.iter() {
            if d.name == name {
                return Err(String::from("device with this name already exists"));
            }
        }

        let dev = device::Device { name };

        self.devices.push(dev);

        Ok(())
    }

    pub fn list_devices(&self) -> &[device::Device] {
        &self.devices
    }

    pub fn get_device(&self, index: usize) -> Option<&device::Device> {
        self.devices.get(index)
    }
}
