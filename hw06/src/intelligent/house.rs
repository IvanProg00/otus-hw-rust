use crate::intelligent::{error, room};

pub trait IntelligentHouse {
    fn new(name: String) -> Self;
    fn get_name(&self) -> &str;
    fn push_room(&mut self, name: String) -> Result<(), error::IntelligentError>;
    fn list_rooms(&self) -> &[room::Room];
    fn list_rooms_mut(&mut self) -> &mut [room::Room];
    fn get_room(&self, index: usize) -> Option<&room::Room>;
    fn get_room_mut(&mut self, index: usize) -> Option<&mut room::Room>;
    fn delete_room(&mut self, index: usize) -> Result<(), error::IntelligentError>;
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

    fn push_room(&mut self, name: String) -> Result<(), error::IntelligentError> {
        for r in self.rooms.iter() {
            if r.name == name {
                return Err(error::IntelligentError {
                    err: error::IntelligentErrors::RoomWithThisNameAlreadyExists,
                });
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

    fn delete_room(&mut self, index: usize) -> Result<(), error::IntelligentError> {
        if index >= self.rooms.len() {
            return Err(error::IntelligentError {
                err: error::IntelligentErrors::RoomNotFound,
            });
        }
        self.rooms.remove(index);
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intelligent::room::Room;

    #[test]
    fn test_get_name() {
        let tests = [
            ("house name 1", "house name 1"),
            ("world", "world"),
            ("rust", "rust"),
        ];

        for (i, &(name, expected)) in tests.iter().enumerate() {
            let r = House {
                name: String::from(name),
                rooms: Vec::new(),
            };

            assert_eq!(r.get_name(), expected, "test-{}", i);
        }
    }

    #[test]
    fn test_push_room() {
        let tests = [(
            vec![Room {
                name: String::from("room 84394"),
                devices: Vec::new(),
            }],
            "room 14843",
        )];

        for (rooms, push_room) in tests {
            let mut r = House {
                name: String::new(),
                rooms,
            };

            let res = r.push_room(String::from(push_room));
            assert!(res.is_ok());
        }
    }

    #[test]
    fn test_push_room_error_exists() {
        let tests = [(
            vec![Room {
                name: String::from("room 74894"),
                devices: Vec::new(),
            }],
            "room 74894",
            error::IntelligentErrors::RoomWithThisNameAlreadyExists,
        )];

        for (rooms, push_room, expected_err) in tests {
            let mut h = House {
                name: String::new(),
                rooms,
            };

            let res = h.push_room(String::from(push_room));
            assert!(res.is_err());

            let expected_err = error::IntelligentError { err: expected_err };
            assert_eq!(res.unwrap_err().to_string(), expected_err.to_string());
        }
    }

    #[test]
    fn test_delete_room() {
        let tests = [
            (
                vec![Room {
                    name: String::from("room 48"),
                    devices: Vec::new(),
                }],
                0,
                0,
            ),
            (
                vec![
                    Room {
                        name: String::from("room 14"),
                        devices: Vec::new(),
                    },
                    Room {
                        name: String::from("room 48"),
                        devices: Vec::new(),
                    },
                ],
                0,
                1,
            ),
            (
                vec![
                    Room {
                        name: String::from("room 94"),
                        devices: Vec::new(),
                    },
                    Room {
                        name: String::from("room 41"),
                        devices: Vec::new(),
                    },
                    Room {
                        name: String::from("room 39"),
                        devices: Vec::new(),
                    },
                ],
                1,
                2,
            ),
        ];

        for (rooms, id, exp_len) in tests {
            let mut h = House {
                name: String::new(),
                rooms,
            };

            let res = h.delete_room(id);
            assert!(res.is_ok());

            assert_eq!(h.rooms.len(), exp_len);
        }
    }

    #[test]
    fn test_delete_room_error() {
        let tests = [
            (
                vec![Room {
                    name: String::from("room 48"),
                    devices: Vec::new(),
                }],
                1,
                1,
                error::IntelligentErrors::RoomNotFound,
            ),
            (
                vec![
                    Room {
                        name: String::from("room 94"),
                        devices: Vec::new(),
                    },
                    Room {
                        name: String::from("room 41"),
                        devices: Vec::new(),
                    },
                    Room {
                        name: String::from("room 39"),
                        devices: Vec::new(),
                    },
                ],
                4,
                3,
                error::IntelligentErrors::RoomNotFound,
            ),
        ];

        for (rooms, id, exp_len, exp_err) in tests {
            let mut h = House {
                name: String::new(),
                rooms,
            };

            let res = h.delete_room(id);
            assert!(res.is_err());

            assert_eq!(h.rooms.len(), exp_len);

            let expected_err = error::IntelligentError { err: exp_err };
            assert_eq!(res.unwrap_err().to_string(), expected_err.to_string());
        }
    }
}
