pub struct Device {
    pub name: String,
}

impl Device {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let tests = [
            ("device name 1", "device name 1"),
            ("world", "world"),
            ("rust", "rust"),
        ];

        for (i, &(name, expected)) in tests.iter().enumerate() {
            let d = Device {
                name: String::from(name),
            };

            assert_eq!(d.get_name(), expected, "test-{}", i);
        }
    }
}
