use sea_orm_migration::prelude::*;

use super::m20230109_000001_create_album_table::Album;
use super::m20230109_000003_create_artist_table::Artist;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230109_000004_create_album_artist_mtm"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlbumArtistMtm::Table)
                    .col(
                        ColumnDef::new(AlbumArtistMtm::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AlbumArtistMtm::AlbumId).integer().not_null())
                    .col(
                        ColumnDef::new(AlbumArtistMtm::ArtistId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-album_artist_mtm-album_id")
                            .from(AlbumArtistMtm::Table, AlbumArtistMtm::AlbumId)
                            .to(Album::Table, Album::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-album_artist_mtm-artist_id")
                            .from(AlbumArtistMtm::Table, AlbumArtistMtm::ArtistId)
                            .to(Artist::Table, Artist::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(AlbumArtistMtm::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum AlbumArtistMtm {
    Table,
    Id,
    AlbumId,
    ArtistId,
}
