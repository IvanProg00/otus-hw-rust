use crate::intelligent::{device, error};

pub struct Room {
    pub name: String,
    pub devices: Vec<device::Device>,
}

impl Room {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn push_device(&mut self, name: String) -> Result<(), error::IntelligentError> {
        for d in self.devices.iter() {
            if d.name == name {
                return Err(error::IntelligentError {
                    err: error::IntelligentErrors::DeviceWithThisNameAlreadyExists,
                });
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

    pub fn delete_device(&mut self, index: usize) -> Result<(), error::IntelligentError> {
        if index >= self.devices.len() {
            return Err(error::IntelligentError {
                err: error::IntelligentErrors::DeviceNotFound,
            });
        }
        self.devices.remove(index);
        Ok(())
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
            assert!(res.is_ok());
        }
    }

    #[test]
    fn test_push_device_error_exists() {
        let tests = [(
            vec![Device {
                name: String::from("device 74894"),
            }],
            "device 74894",
            error::IntelligentErrors::DeviceWithThisNameAlreadyExists,
        )];

        for (devices, push_device, expected_err) in tests {
            let mut r = Room {
                name: String::new(),
                devices,
            };

            let res = r.push_device(String::from(push_device));
            assert!(res.is_err());

            let expected_err = error::IntelligentError { err: expected_err };
            assert_eq!(res.unwrap_err().to_string(), expected_err.to_string());
        }
    }

    #[test]
    fn test_delete_device() {
        let tests = [
            (
                vec![Device {
                    name: String::from("device 33"),
                }],
                0,
                0,
            ),
            (
                vec![
                    Device {
                        name: String::from("room 14"),
                    },
                    Device {
                        name: String::from("room 48"),
                    },
                ],
                0,
                1,
            ),
            (
                vec![
                    Device {
                        name: String::from("room 94"),
                    },
                    Device {
                        name: String::from("room 41"),
                    },
                    Device {
                        name: String::from("room 39"),
                    },
                ],
                1,
                2,
            ),
        ];

        for (devices, id, exp_len) in tests {
            let mut h = Room {
                name: String::new(),
                devices,
            };

            let res = h.delete_device(id);
            assert!(res.is_ok());

            assert_eq!(h.devices.len(), exp_len);
        }
    }

    #[test]
    fn test_delete_room_error() {
        let tests = [
            (
                vec![Device {
                    name: String::from("room 48"),
                }],
                1,
                1,
                error::IntelligentErrors::DeviceNotFound,
            ),
            (
                vec![
                    Device {
                        name: String::from("room 94"),
                    },
                    Device {
                        name: String::from("room 41"),
                    },
                    Device {
                        name: String::from("room 39"),
                    },
                ],
                4,
                3,
                error::IntelligentErrors::DeviceNotFound,
            ),
        ];

        for (devices, id, exp_len, exp_err) in tests {
            let mut h = Room {
                name: String::new(),
                devices,
            };

            let res = h.delete_device(id);
            assert!(res.is_err());

            assert_eq!(h.devices.len(), exp_len);

            let expected_err = error::IntelligentError { err: exp_err };
            assert_eq!(res.unwrap_err().to_string(), expected_err.to_string());
        }
    }
}
