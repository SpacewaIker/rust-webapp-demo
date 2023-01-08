use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum SongError {
    CreateError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub length_secs: u32,
}

impl Song {
    pub fn new(name: String, length_secs: u32) -> Result<Self, SongError> {
        if name.is_empty() {
            Err(SongError::CreateError("Name cannot be empty".to_string()))
        } else if length_secs == 0 {
            Err(SongError::CreateError("Length cannot be 0".to_string()))
        } else {
            Ok(Song {
                id: None,
                name,
                length_secs,
            })
        }
    }
}
