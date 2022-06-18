use crate::intelligent::room;

pub trait IntelligentHouse {
    fn new(name: String) -> Self;
    fn get_name(&self) -> &str;
    fn push_room(&mut self, name: String) -> Result<(), String>;
    fn list_rooms(&self) -> &[room::Room];
    fn list_rooms_mut(&mut self) -> &mut [room::Room];
    fn get_room(&self, index: usize) -> Option<&room::Room>;
    fn get_room_mut(&mut self, index: usize) -> Option<&mut room::Room>;
    fn create_report(&self) -> String;
}

pub struct House {
    name: String,
    rooms: Vec<room::Room>,
}

impl IntelligentHouse for House {
    fn new(name: String) -> Self {
        House {
            name,
            rooms: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn push_room(&mut self, name: String) -> Result<(), String> {
        for r in self.rooms.iter() {
            if r.name == name {
                return Err(String::from("room with this name already exists"));
            }
        }

        let room = room::Room {
            name,
            devices: Vec::new(),
        };
        self.rooms.push(room);

        Ok(())
    }

    fn list_rooms(&self) -> &[room::Room] {
        &self.rooms
    }

    fn list_rooms_mut(&mut self) -> &mut [room::Room] {
        &mut self.rooms
    }

    fn get_room(&self, index: usize) -> Option<&room::Room> {
        self.rooms.get(index)
    }

    fn get_room_mut(&mut self, index: usize) -> Option<&mut room::Room> {
        self.rooms.get_mut(index)
    }

    fn create_report(&self) -> String {
        let mut res = format!("House \"{}\":", self.get_name());

        for r in self.list_rooms().iter() {
            res.push('\n');
            res.push_str(format!("\tRoom \"{}\":", r.get_name()).as_str());

            if r.list_devices().is_empty() {
                res.push_str("\n\t\tdevices not found");
                continue;
            }

            for d in r.list_devices().iter() {
                res.push('\n');
                res.push_str(format!("\t\tDevice \"{}\"", d.get_name()).as_str());
            }
        }

        res
    }
}
