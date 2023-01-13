use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230109_000003_create_artist_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Genre::Genre)
                    .values([
                        Genre::Metal,
                        Genre::Classical,
                        Genre::Rock,
                        Genre::Pop,
                        Genre::Jazz,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Artist::Table)
                    .col(
                        ColumnDef::new(Artist::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Artist::Name).string().not_null())
                    .col(ColumnDef::new(Artist::DateFormed).date().not_null())
                    .col(ColumnDef::new(Artist::Genre).enumeration(
                        Genre::Genre,
                        vec![
                            Genre::Metal,
                            Genre::Classical,
                            Genre::Rock,
                            Genre::Pop,
                            Genre::Jazz,
                        ],
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Artist::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().if_exists().name(Genre::Genre).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Genre {
    Genre,
    Metal,
    Classical,
    Rock,
    Pop,
    Jazz,
}

#[derive(Iden)]
pub enum Artist {
    Table,
    Id,
    Name,
    DateFormed,
    Genre,
}
