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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intelligent::device::Device;

    #[test]
    fn test_get_name() {
        let tests = [
            ("room name 1", "room name 1"),
            ("world", "world"),
            ("rust", "rust"),
        ];

        for (i, &(name, expected)) in tests.iter().enumerate() {
            let r = Room {
                name: String::from(name),
                devices: Vec::new(),
            };

            assert_eq!(r.get_name(), expected, "test-{}", i);
        }
    }

    #[test]
    fn test_push_device() {
        let tests = [(
            vec![Device {
                name: String::from("device 59439"),
            }],
            "device 43948",
        )];

        for (devices, push_device) in tests {
            let mut r = Room {
                name: String::new(),
                devices,
            };

            let res = r.push_device(String::from(push_device));
            assert_eq!(res, Ok(()));
        }
    }

    #[test]
    fn test_push_device_error_exists() {
        let tests = [(
            vec![Device {
                name: String::from("device 74894"),
            }],
            "device 74894",
        )];

        for (devices, push_device) in tests {
            let mut r = Room {
                name: String::new(),
                devices,
            };

            let res = r.push_device(String::from(push_device));
            assert!(res.is_err());
            assert_eq!(
                res,
                Err(String::from("device with this name already exists"))
            )
        }
    }
}
