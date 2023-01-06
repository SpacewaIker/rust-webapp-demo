use chrono::NaiveDate;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub length_secs: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub date_published: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub genre: MusicGenre,
    pub date_formed: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MusicGenre {
    Rock,
    Pop,
    Jazz,
    Classical,
    Country,
    Folk,
    Blues,
    Electronic,
    HipHop,
    RnB,
    Soul,
    Reggae,
    Punk,
    Metal,
    Other(String),
}