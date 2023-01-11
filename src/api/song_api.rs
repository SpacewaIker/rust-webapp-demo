use crate::entities::{prelude::*, song};
use rocket::{http::Status, response::status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[post("/", data = "<song>")]
pub async fn create_song(
    db: &State<DatabaseConnection>,
    song: Json<song::Model>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let album = Album::find_by_id(song.album_id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching album in DB"))?;

    if song.name.is_empty() || song.length_secs <= 0 || album.is_none() {
        return Err((Status::BadRequest, "Invalid input"));
    }

    let new_song = song::ActiveModel {
        name: ActiveValue::Set(song.name.to_owned()),
        length_secs: ActiveValue::Set(song.length_secs),
        album_id: ActiveValue::Set(song.album_id),
        ..Default::default()
    };

    let song = Song::insert(new_song)
        .exec(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error inserting song in DB"))?;

    Ok((Status::Created, song.last_insert_id.to_string()))
}

#[get("/<id>")]
pub async fn get_song_by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, Json<song::Model>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let song = Song::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching song in DB"))?;

    Ok((
        Status::Found,
        Json(song.ok_or((Status::NotFound, "Song not found"))?),
    ))
}

#[put("/<id>", data = "<song>")]
pub async fn update_song(
    db: &State<DatabaseConnection>,
    id: i32,
    song: Json<song::Model>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let updated_song = song::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(song.name.to_owned()),
        length_secs: ActiveValue::Set(song.length_secs),
        album_id: ActiveValue::Set(song.album_id),
        ..Default::default()
    };

    updated_song
        .update(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error updating song"))?;

    Ok((Status::Accepted, id.to_string()))
}

#[delete("/<id>")]
pub async fn delete_song(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let res = Song::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error deleting song"))?;

    match res.rows_affected {
        1 => Ok((Status::Ok, id.to_string())),
        _ => Err((Status::NotFound, "Song not found")),
    }
}

#[get("/all")]
pub async fn get_all_songs(
    db: &State<DatabaseConnection>,
) -> Result<(Status, Json<Vec<song::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let songs = Song::find()
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching songs in DB"))?;

    Ok((Status::Ok, Json(songs)))
}
