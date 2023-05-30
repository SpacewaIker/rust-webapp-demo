use chrono::Local;
use entities::{album, album_artist_mtm, artist, prelude::*};
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

/// Create a new artist
///
/// This endpoint requires a JSON body containing an [`artist::Model`] without an id.
#[post("/", data = "<artist>")]
pub async fn create_artist(
    db: &State<DatabaseConnection>,
    artist: Json<artist::Model>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    if artist.name.is_empty() || artist.date_formed > Local::now().date_naive() {
        info!("Invalid input");
        return Err((Status::BadRequest, "Invalid input"));
    }

    let new_artist = artist::ActiveModel {
        name: ActiveValue::Set(artist.name.to_owned()),
        date_formed: ActiveValue::Set(artist.date_formed),
        genre: ActiveValue::Set(artist.genre.to_owned()),
        ..Default::default()
    };

    let artist = Artist::insert(new_artist)
        .exec(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error inserting artist in DB"))?;

    info!("Inserted artist with id: {}", artist.last_insert_id);

    Ok((Status::Created, artist.last_insert_id.to_string()))
}

/// Get an artist by its id
///
/// This endpoint returns an [`artist::Model`] with the given id, if it exists.
#[get("/<id>")]
pub async fn get_artist_by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, Json<artist::Model>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let artist = Artist::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artist in DB"))?;

    let artist = artist.ok_or((Status::NotFound, "Artist not found"))?;
    info!("Found artist with id: {}", artist.id);
    Ok((Status::Found, Json(artist)))
}

/// Update an artist with the given id
///
/// This endpoint requires a JSON body containing a [`artist::Model`] with all of the fields set.
#[put("/<id>", data = "<artist>")]
pub async fn update_artist(
    db: &State<DatabaseConnection>,
    id: i32,
    artist: Json<artist::Model>,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let updated_artist = artist::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(artist.name.to_owned()),
        date_formed: ActiveValue::Set(artist.date_formed.to_owned()),
        genre: ActiveValue::Set(artist.genre.to_owned()),
        ..Default::default()
    };

    info!(
        "About to update artist with id {} to: {:?}",
        id, updated_artist
    );

    updated_artist
        .update(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error updating artist"))?;

    info!("Updated artist with id: {}", id);

    Ok((Status::Accepted, id.to_string()))
}

/// Delete an artist with the given id
///
/// This endpoint returns the id of the deleted artist, if it exists.
#[delete("/<id>")]
pub async fn delete_artist(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, String), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let album_ids = AlbumArtistMtm::find()
        .filter(album_artist_mtm::Column::ArtistId.eq(id))
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error finding artist's albums"))?
        .into_iter()
        .map(|album_artist| album_artist.album_id);

    let res = Artist::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error deleting artist"))?;

    let ret = match res.rows_affected {
        1 => {
            info!("Deleted artist with id {}", id);
            Ok((Status::Ok, id.to_string()))
        }
        _ => return Err((Status::NotFound, "Artist not found")),
    };

    for album_id in album_ids {
        let no_other_artists = AlbumArtistMtm::find()
            .filter(album_artist_mtm::Column::AlbumId.eq(album_id))
            .all(db)
            .await
            .map_err(|_| (Status::InternalServerError, "Error finding album-artists"))?
            .is_empty();

        if no_other_artists {
            Album::delete_by_id(album_id)
                .exec(db)
                .await
                .map_err(|_| (Status::InternalServerError, "Error deleting albums"))?;
        }
    }

    ret
}

/// Get all artists
///
/// This endpoint returns a list of all artists in the database.
#[get("/all")]
pub async fn get_all_artists(
    db: &State<DatabaseConnection>,
) -> Result<(Status, Json<Vec<artist::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let artists = Artist::find()
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artists in DB"))?;

    info!("Found {} artists", artists.len());
    Ok((Status::Ok, Json(artists)))
}

/// Get all albums of an artist
#[get("/albums/<id>")]
pub async fn get_albums(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<(Status, Json<Vec<album::Model>>), (Status, &str)> {
    let db = db as &DatabaseConnection;

    let artist = Artist::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching artist in DB"))?;

    if artist.is_none() {
        info!("Artist not found");
        return Err((Status::NotFound, "Artist not found"));
    }

    let album_ids = AlbumArtistMtm::find()
        .filter(album_artist_mtm::Column::ArtistId.eq(id))
        .all(db)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                "Error fetching album-artists in DB",
            )
        })?
        .into_iter()
        .map(|album_artist| album_artist.album_id);

    let albums = Album::find()
        .filter(album::Column::Id.is_in(album_ids))
        .all(db)
        .await
        .map_err(|_| (Status::InternalServerError, "Error fetching albums in DB"))?;

    info!("Found {} artists", albums.len());
    Ok((Status::Ok, Json(albums)))
}
