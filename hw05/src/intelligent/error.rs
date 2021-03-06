use std::{error::Error, fmt};

#[derive(Debug)]
pub enum IntelligentErrors {
    RoomWithThisNameAlreadyExists,
    DeviceWithThisNameAlreadyExists,
}

#[derive(Debug)]
pub struct IntelligentError {
    pub err: IntelligentErrors,
}

impl fmt::Display for IntelligentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.err {
            IntelligentErrors::RoomWithThisNameAlreadyExists => {
                write!(f, "room with this name already exists")
            }
            IntelligentErrors::DeviceWithThisNameAlreadyExists => {
                write!(f, "device with this name already exists")
            }
        }
    }
}

impl Error for IntelligentError {}
