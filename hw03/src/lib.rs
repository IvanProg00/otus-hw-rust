pub struct House<'a> {
    name: String,
    rooms: Vec<Room<'a>>,
}

pub struct Room<'a> {
    name: String,
    devices: Vec<&'a Device>,
}

pub struct Device {
    name: String,
}

impl<'a> House<'a> {
    pub fn new(name: String) -> Self {
        House {
            name,
            rooms: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn push_room(&mut self, name: String) -> Result<(), String> {
        for r in self.rooms.iter() {
            if r.name == name {
                return Err(String::from("room with this name already exists"));
            }
        }

        let room = Room {
            name,
            devices: Vec::new(),
        };
        self.rooms.push(room);

        Ok(())
    }

    pub fn list_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }
}

impl<'a> Room<'a> {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn push_device(&mut self, name: String) -> Result<(), String> {
        for d in self.devices.iter() {
            if d.name == name {
                return Err(String::from("device with this name already exists"));
            }
        }

        let dev = Device { name: name.clone() };

        self.devices.push(&dev);

        Ok(())
    }

    pub fn list_devices(&self) -> &Vec<&Device> {
        &self.devices
    }
}

impl Device {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
