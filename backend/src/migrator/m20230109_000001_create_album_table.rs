use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230109_000001_create_album_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Album::Table)
                    .col(
                        ColumnDef::new(Album::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Album::Name).string().not_null())
                    .col(ColumnDef::new(Album::DatePublished).date().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Album::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Album {
    Table,
    Id,
    Name,
    DatePublished,
}
