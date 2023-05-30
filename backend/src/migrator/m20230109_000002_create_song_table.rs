use sea_orm_migration::prelude::*;

use super::m20230109_000001_create_album_table::Album;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230109_000002_create_song_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .col(
                        ColumnDef::new(Song::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Song::Name).string().not_null())
                    .col(ColumnDef::new(Song::LengthSecs).unsigned().not_null())
                    .col(ColumnDef::new(Song::AlbumId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-song-album_id")
                            .from(Song::Table, Song::AlbumId)
                            .to(Album::Table, Album::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Song::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Song {
    Table,
    Id,
    Name,
    LengthSecs,
    AlbumId,
}

