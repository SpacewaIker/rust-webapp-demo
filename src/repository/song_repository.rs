extern crate dotenv;

use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use crate::model::song::Song;
use crate::repository::MongoRepo;
use rocket::http::Status;

impl MongoRepo {
    pub fn create_song(&self, song: Song) -> Result<InsertOneResult, Status> {
        let doc = Song::new(song.name, song.length_secs).map_err(|_| Status::BadRequest)?;
        self.collection
            .insert_one(doc, None)
            .map_err(|_| Status::InternalServerError)
    }

    pub fn get_song(&self, id: &String) -> Result<Song, Status> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
        let filter = doc! {"_id": obj_id};
        self
            .collection
            .find_one(filter, None)
            .map_err(|_| Status::InternalServerError)?
            .ok_or(Status::NotFound)
    }

    pub fn update_song(&self, id: &String, song: Song) -> Result<UpdateResult, Status> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
        let filter = doc! {"_id": obj_id};
        let song = Song::new(song.name, song.length_secs).map_err(|_| Status::BadRequest)?;
        let new_doc = doc! {
            "$set":
                {
                    "id": song.id,
                    "name": song.name,
                    "length_secs": song.length_secs,
                },
        };
        self
            .collection
            .update_one(filter, new_doc, None)
            .map_err(|_| Status::InternalServerError)
    }

    pub fn delete_song(&self, id: &String) -> Result<DeleteResult, Status> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
        let filter = doc! {"_id": obj_id};
        self
            .collection
            .delete_one(filter, None)
            .map_err(|_| Status::InternalServerError)
    }

    pub fn get_all_songs(&self) -> Result<Vec<Song>, Status> {
        let mut songs = self
            .collection
            .find(None, None)
            .map_err(|_| Status::InternalServerError)?
            .map(|doc| doc.ok())
            .collect::<Vec<Option<Song>>>();
        songs.retain(|song| song.is_some());
        Ok(songs.into_iter().map(|song| song.unwrap()).collect())
    }
}
