mod api;
mod model;
mod repository;

#[macro_use]
extern crate rocket;

use repository::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .mount(
            "/song",
            routes![
                api::song_api::create_song,
                api::song_api::get_song,
                api::song_api::update_song,
                api::song_api::delete_song,
                api::song_api::get_all_songs,
            ],
        )
        .manage(db)
}
