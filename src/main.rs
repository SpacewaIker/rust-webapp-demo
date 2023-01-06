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
            "/",
            routes![
                api::create_song,
                api::get_song,
                api::update_song,
                api::delete_song,
                api::get_all_songs,
            ],
        )
        .manage(db)
}
