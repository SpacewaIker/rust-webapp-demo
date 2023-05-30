use chrono::Local;
use entities::{album, album_artist_mtm, artist, prelude::*, song};
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

/// Create a new album for the given artist
///
/// This endpoint requires a JSON body containing an [`album::Model`] without an id.
#[post("/<artist_id>", data = "<album>")]
pub async fn create_album(
    db: &State<DatabaseConnection>,
    album: Json<album::Model>,
    artist_id: i32,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let artist = Artist::find_by_id(artist_id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artist in DB"))?;

    if album.name.is_empty() || album.date_published > Local::now().date_naive() || artist.is_none()
    {
        info!("Invalid input");
        return Err((Status::BadRequest, "Invalid input"));
    }

    let new_album = album::ActiveModel {
        name: ActiveValue::Set(album.name.to_owned()),
        date_published: ActiveValue::Set(album.date_published),
        ..Default::default()
    };

    let album = Album::insert(new_album)
        .exec(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error inserting album in DB"))?;

    info!("Inserted album with id: {}", album.last_insert_id);

    let new_artist_album = album_artist_mtm::ActiveModel {
        artist_id: ActiveValue::Set(artist_id),
        album_id: ActiveValue::Set(album.last_insert_id),
        ..Default::default()
    };

    let artist_album = AlbumArtistMtm::insert(new_artist_album)
        .exec(db)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                "Error inserting artist_album in DB",
            )
        })?;

    info!(
        "Inserted album-artist relation with id: {}",
        artist_album.last_insert_id
    );

    Ok((Status::Created, album.last_insert_id.to_string()))
}

/// Get an album by its id
///
/// This endpoint returns an [`album::Model`] with the given id, if it exists.
#[get("/<id>")]
pub async fn get_album_by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, Json<album::Model>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let album = Album::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching album in DB"))?;

    let album = album.ok_or((Status::NotFound, "Album not found"))?;
    info!("Found album with id: {}", album.id);
    Ok((Status::Found, Json(album)))
}

/// Update an album with the given id
///
/// This endpoint requires a JSON body containing a [`album::Model`] with all of the fields set.
#[put("/<id>", data = "<album>")]
pub async fn update_album(
    db: &State<DatabaseConnection>,
    id: i32,
    album: Json<album::Model>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let updated_album = album::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(album.name.to_owned()),
        date_published: ActiveValue::Set(album.date_published.to_owned()),
        ..Default::default()
    };

    info!(
        "About to update album with id {} to: {:?}",
        id, updated_album
    );

    updated_album
        .update(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error updating album"))?;

    info!("Updated album with id: {}", id);

    Ok((Status::Accepted, id.to_string()))
}

/// Delete an album with the given id
///
/// This endpoint returns the id of the deleted album, if it exists.
#[delete("/<id>")]
pub async fn delete_album(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let res = Album::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error deleting album"))?;

    match res.rows_affected {
        1 => {
            info!("Deleted album with id {}", id);
            Ok((Status::Ok, id.to_string()))
        }
        _ => Err((Status::NotFound, "Album not found")),
    }
}

/// Get all albums
///
/// This endpoint returns a list of all albums in the database.
#[get("/all")]
pub async fn get_all_albums(
    db: &State<DatabaseConnection>,
) -> Result<(Status, Json<Vec<album::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let albums = Album::find()
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching albums in DB"))?;

    info!("Found {} albums", albums.len());
    Ok((Status::Ok, Json(albums)))
}

/// Add an artist to the album with the given id
///
/// This endpoint requires a JSON body containing a list of artist ids.
#[post("/artist/<id>", data = "<artist_ids>")]
pub async fn add_artist(
    db: &State<DatabaseConnection>,
    id: i32,
    artist_ids: Json<Vec<i32>>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let album = Album::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching album in DB"))?;

    if album.is_none() {
        info!("Album not found");
        return Err((Status::NotFound, "Album not found"));
    }

    let artist_ids = artist_ids.into_inner();
    let artists = Artist::find()
        .filter(artist::Column::Id.is_in(artist_ids.clone()))
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artists in DB"))?;

    if artists.len() != artist_ids.len() {
        info!("Not all artists were found");
        return Err((Status::BadRequest, "Invalid list of artist ids"));
    }

    for artist_id in artist_ids {
        let new_artist_album = album_artist_mtm::ActiveModel {
            artist_id: ActiveValue::Set(artist_id),
            album_id: ActiveValue::Set(id),
            ..Default::default()
        };

        let artist_album = AlbumArtistMtm::insert(new_artist_album)
            .exec(db)
            .await
            .map_err(|_| {
                (
                    Status::InternalServerError,
                    "Error inserting artist_album in DB",
                )
            })?;

        info!(
            "Inserted album-artist relation with id: {}",
            artist_album.last_insert_id
        );
    }

    Ok((Status::Created, id.to_string()))
}

/// Get all artists for the album with the given id
///
/// This endpoint returns a list of [`artist::Model`] for the album with the given id.
#[get("/artist/<id>")]
pub async fn get_artists(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, Json<Vec<artist::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let album = Album::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching album in DB"))?;

    if album.is_none() {
        info!("Album not found");
        return Err((Status::NotFound, "Album not found"));
    }

    let artists_ids = AlbumArtistMtm::find()
        .filter(album_artist_mtm::Column::AlbumId.eq(id))
        .all(db)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                "Error fetching album-artists in DB",
            )
        })?
        .into_iter()
        .map(|album_artist| album_artist.artist_id);

    let artists = Artist::find()
        .filter(artist::Column::Id.is_in(artists_ids))
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artists in DB"))?;

    info!("Found {} artists", artists.len());
    Ok((Status::Ok, Json(artists)))
}

/// Remove an artist from the album with the given id
///
/// This endpoint requires a JSON body containing a list of artist ids.
#[delete("/artist/<id>", data = "<artist_ids>")]
pub async fn remove_artist(
    db: &State<DatabaseConnection>,
    id: i32,
    artist_ids: Json<Vec<i32>>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let album = Album::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching album in DB"))?;

    if album.is_none() {
        info!("Album not found");
        return Err((Status::NotFound, "Album not found"));
    }

    let artist_ids = artist_ids.into_inner();
    let artists = Artist::find()
        .filter(artist::Column::Id.is_in(artist_ids.clone()))
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artists in DB"))?;

    if artists.len() != artist_ids.len() {
        info!("Not all artists were found");
        return Err((Status::BadRequest, "Invalid list of artist ids"));
    }

    let res = AlbumArtistMtm::delete_many()
        .filter(album_artist_mtm::Column::ArtistId.is_in(artist_ids))
        .filter(album_artist_mtm::Column::AlbumId.eq(id))
        .exec(db)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                "Error deleting artist_album in DB",
            )
        })?;

    info!("Deleted {} album-artist relations", res.rows_affected);
    Ok((Status::Ok, id.to_string()))
}

/// Get all songs of an album
#[get("/songs/<id>")]
pub async fn get_songs(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, Json<Vec<song::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let songs = Song::find()
        .filter(song::Column::AlbumId.eq(id))
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching songs in DB"))?;

    info!("Found {} songs", songs.len());
    Ok((Status::Found, Json(songs)))
}
