use entities::{prelude::*, song};
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

/// Create a new song
/// 
/// This endpoint requires a JSON body containing a [`song::Model`] without an id.
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
        info!("Invalid input");
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

    info!("Inserted song with id: {}", song.last_insert_id);
    Ok((Status::Created, song.last_insert_id.to_string()))
}

/// Get a song by its id
/// 
/// This endpoint returns a [`song::Model`] with the given id, if it exists.
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

    let song = song.ok_or((Status::NotFound, "Song not found"))?;
    info!("Found song with id: {}", song.id);
    Ok((Status::Found, Json(song)))
}

/// Update a song with the given id
/// 
/// This endpoint requires a JSON body containing a [`song::Model`] with all of the fields set.
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

    info!("About to update song with id {} to: {:?}", id, updated_song);

    updated_song
        .update(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error updating song"))?;

    info!("Updated song with id: {}", id);

    Ok((Status::Accepted, id.to_string()))
}

/// Delete a song with the given id
/// 
/// This endpoint returns the id of the deleted song, if it exists.
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
        1 => {
            info!("Deleted song with id {}", id);
            Ok((Status::Ok, id.to_string()))
        }
        _ => Err((Status::NotFound, "Song not found")),
    }
}

/// Get all songs
/// 
/// This endpoint returns a list of all songs in the database.
#[get("/all")]
pub async fn get_all_songs(
    db: &State<DatabaseConnection>,
) -> Result<(Status, Json<Vec<song::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let songs = Song::find()
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching songs in DB"))?;

    info!("Found {} songs", songs.len());
    Ok((Status::Ok, Json(songs)))
}
