mod song_repository;

use crate::model::song::Song;
use mongodb::sync::{Client, Collection};
use dotenv::dotenv;
use std::env;

pub struct MongoRepo {
    collection: Collection<Song>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Song> = db.collection("Song");
        MongoRepo { collection: col }
    }
}