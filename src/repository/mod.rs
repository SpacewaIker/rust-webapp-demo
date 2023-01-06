use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};

use crate::model::Song;

pub struct MongoRepo {
    collection: Collection<Song>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Song> = db.collection("Song");
        MongoRepo { collection: col }
    }

    pub fn create_song(&self, song: Song) -> Result<InsertOneResult, Error> {
        let doc = Song { id: None, ..song };
        let song = self
            .collection
            .insert_one(doc, None)
            .ok()
            .expect("Error creating song");
        Ok(song)
    }

    pub fn get_song(&self, id: &String) -> Result<Song, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let song = self
            .collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting song");
        Ok(song.unwrap())
    }

    pub fn update_song(&self, id: &String, song: Song) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": song.id,
                    "name": song.name,
                    "length_secs": song.length_secs,
                },
        };
        let updated_doc = self
            .collection
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating song");
        Ok(updated_doc)
    }

    pub fn delete_song(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let deleted_doc = self
            .collection
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting song");
        Ok(deleted_doc)
    }

    pub fn get_all_songs(&self) -> Result<Vec<Song>, Error> {
        let songs = self
            .collection
            .find(None, None)
            .ok()
            .expect("Error getting all songs")
            .map(|doc| doc.unwrap())
            .collect();
        Ok(songs)
    }
}
