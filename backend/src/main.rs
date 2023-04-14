pub mod api;
mod migrator;
// pub mod entities;

// #[cfg(test)] mod tests;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::{MigratorTrait, SchemaManager};
use std::env;

pub async fn set_up_db() -> DatabaseConnection {
    let url = match env::var("POSTGRES_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    Database::connect(url)
        .await
        .expect("Error connecting to database")
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let db = set_up_db().await;

    let schema_manager = SchemaManager::new(&db);
    migrator::Migrator::refresh(&db).await.unwrap();
    assert!(schema_manager.has_table("song").await.unwrap());
    assert!(schema_manager.has_table("album").await.unwrap());
    assert!(schema_manager.has_table("artist").await.unwrap());
    assert!(schema_manager.has_table("album_artist_mtm").await.unwrap());

    rocket::build()
        .manage(db)
        .mount(
            "/song",
            routes![
                api::song_api::create_song,
                api::song_api::get_song_by_id,
                api::song_api::update_song,
                api::song_api::delete_song,
                api::song_api::get_all_songs,
            ],
        )
        .mount(
            "/album",
            routes![
                api::album_api::create_album,
                api::album_api::get_album_by_id,
                api::album_api::update_album,
                api::album_api::delete_album,
                api::album_api::get_all_albums,
                api::album_api::add_artist,
                api::album_api::get_artists,
                api::album_api::remove_artist,
            ],
        )
        .mount(
            "/artist",
            routes![
                api::artist_api::create_artist,
                api::artist_api::get_artist_by_id,
                api::artist_api::update_artist,
                api::artist_api::delete_artist,
                api::artist_api::get_all_artists,
            ],
        )
}
