use crate::{model::song::Song, repository::MongoRepo};
use mongodb::results::{DeleteResult, InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/", data = "<song>")]
pub fn create_song(
    db: &State<MongoRepo>,
    song: Json<Song>,
) -> Result<Json<InsertOneResult>, Status> {
    db.create_song(song.into_inner()).map(|v| Json(v))
}

#[get("/<id>")]
pub fn get_song(db: &State<MongoRepo>, id: String) -> Result<Json<Song>, Status> {
    db.get_song(&id).map(|v| Json(v))
}

#[put("/<id>", data = "<song>")]
pub fn update_song(
    db: &State<MongoRepo>,
    id: String,
    song: Json<Song>,
) -> Result<Json<Song>, Status> {
    let updated_song = db.update_song(&id, song.into_inner())?;
    if updated_song.matched_count == 1 {
        db.get_song(&id).map(|v| Json(v))
    } else {
        Err(Status::NotFound)
    }
}

#[delete("/<id>")]
pub fn delete_song(db: &State<MongoRepo>, id: String) -> Result<Json<DeleteResult>, Status> {
    let delete_result = db.delete_song(&id)?;
    if delete_result.deleted_count == 1 {
        Ok(Json(delete_result))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/all")]
pub fn get_all_songs(db: &State<MongoRepo>) -> Result<Json<Vec<Song>>, Status> {
    db.get_all_songs().map(|v| Json(v))
}
