use crate::{model::*, repository::MongoRepo};
use mongodb::{results::{InsertOneResult, DeleteResult}, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};

#[post("/song", data = "<song>")]
pub fn create_song(db: &State<MongoRepo>, song: Json<Song>) -> Result<Json<InsertOneResult>, Status> {
    let data = Song {
        id: None,
        ..song.into_inner()
    };
    let user_detail = db.create_song(data);
    match user_detail {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/song/<id>")]
pub fn get_song(db: &State<MongoRepo>, id: String) -> Result<Json<Song>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let song = db.get_song(&id);
    match song {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/song/<id>", data = "<song>")]
pub fn update_song(db: &State<MongoRepo>, id: String, song: Json<Song>) -> Result<Json<Song>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let data = Song {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        ..song.into_inner()
    };
    let update_result = db.update_song(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_song(&id);
                match updated_user_info {
                    Ok(v) => Ok(Json(v)),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/song/<id>")]
pub fn delete_song(db: &State<MongoRepo>, id: String) -> Result<Json<DeleteResult>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let delete_result = db.delete_song(&id);
    match delete_result {
        Ok(result) => {
            if result.deleted_count == 1 {
                Ok(Json(result))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/song/all")]
pub fn get_all_songs(db: &State<MongoRepo>) -> Result<Json<Vec<Song>>, Status> {
    let songs = db.get_all_songs();
    match songs {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::InternalServerError),
    }
}