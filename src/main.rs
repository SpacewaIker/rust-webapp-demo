mod api;
mod migrator;
mod entities;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use sea_orm::{DatabaseConnection, Database};
use sea_orm_migration::prelude::{SchemaManager, MigratorTrait};
use std::env;

pub async fn set_up_db() -> DatabaseConnection {
    dotenv().ok();
    let url = match env::var("POSTGRES_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    Database::connect(url).await.expect("Error connecting to database")
}

#[launch]
async fn rocket() -> _ {
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
}
