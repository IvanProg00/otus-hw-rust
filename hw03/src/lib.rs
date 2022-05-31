pub struct House {
    name: String,
    rooms: Vec<Room>,
}

pub struct Room {
    name: String,
    devices: Vec<Device>,
}

pub struct Device {
    name: String,
}

impl House {
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

    pub fn list_rooms(&self) -> &[Room] {
        &self.rooms
    }

    pub fn list_rooms_mut(&mut self) -> &mut [Room] {
        &mut self.rooms
    }

    pub fn get_room(&self, index: usize) -> Option<&Room> {
        self.rooms.get(index)
    }

    pub fn get_room_mut(&mut self, index: usize) -> Option<&mut Room> {
        self.rooms.get_mut(index)
    }
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

        let dev = Device { name };

        self.devices.push(dev);

        Ok(())
    }

    pub fn list_devices(&self) -> &[Device] {
        &self.devices
    }

    pub fn get_device(&self, index: usize) -> Option<&Device> {
        self.devices.get(index)
    }
}

impl Device {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
