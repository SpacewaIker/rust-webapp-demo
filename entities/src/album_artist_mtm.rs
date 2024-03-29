//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

#[cfg(not(target_family = "wasm"))]
use sea_orm::entity::prelude::*;
#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    not(target_family = "wasm"),
    derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)
)]
#[cfg_attr(
    target_family = "wasm",
    derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)
)]
#[cfg_attr(not(target_family = "wasm"), sea_orm(table_name = "album_artist_mtm"))]
pub struct Model {
    #[cfg_attr(not(target_family = "wasm"), sea_orm(primary_key))]
    pub id: i32,
    pub album_id: i32,
    pub artist_id: i32,
}

#[cfg_attr(
    not(target_family = "wasm"),
    derive(Copy, Clone, Debug, EnumIter, DeriveRelation)
)]
#[cfg_attr(target_family = "wasm", derive(Copy, Clone, Debug))]
pub enum Relation {
    #[cfg_attr(
        not(target_family = "wasm"),
        sea_orm(
            belongs_to = "super::album::Entity",
            from = "Column::AlbumId",
            to = "super::album::Column::Id",
            on_update = "NoAction",
            on_delete = "NoAction"
        )
    )]
    Album,
    #[cfg_attr(
        not(target_family = "wasm"),
        sea_orm(
            belongs_to = "super::artist::Entity",
            from = "Column::ArtistId",
            to = "super::artist::Column::Id",
            on_update = "NoAction",
            on_delete = "NoAction"
        )
    )]
    Artist,
}

#[cfg(not(target_family = "wasm"))]
impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def()
    }
}

#[cfg(not(target_family = "wasm"))]
impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

#[cfg(not(target_family = "wasm"))]
impl ActiveModelBehavior for ActiveModel {}
