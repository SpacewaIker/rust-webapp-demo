use crate::api::song_api::*;
use rocket::{local::blocking::Client, Build, Rocket, http::Status};
use sea_orm::{DatabaseBackend, MockDatabase};
use crate::entities::song;

// #[ctor::ctor]
fn setup_backend() -> Rocket<Build> {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![song::Model {
                id: 4,
                name: "My song".to_string(),
                length_secs: 120,
                album_id: 1,
            }],
        ])
        .into_connection();

    rocket::build().manage(db).mount(
        "/song",
        routes![
            create_song,
            get_song_by_id,
            update_song,
            delete_song,
            get_all_songs,
        ],
    )
}

#[test]
fn test() {
    let rocket = setup_backend();
    let client = Client::tracked(rocket).unwrap();

    let req = client.get("/song/1");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Found);
    let json = response.into_json::<song::Model>();
    assert!(json.is_some());
    assert_eq!(json.unwrap(), song::Model {
        id: 0,
        name: "My song".to_string(),
        length_secs: 120,
        album_id: 1,
    });
}
