use sea_orm_migration::prelude::*;

mod m20230109_000001_create_album_table;
mod m20230109_000002_create_song_table;
mod m20230109_000003_create_artist_table;
mod m20230109_000004_create_album_artist_mtm;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230109_000001_create_album_table::Migration),
            Box::new(m20230109_000002_create_song_table::Migration),
            Box::new(m20230109_000003_create_artist_table::Migration),
            Box::new(m20230109_000004_create_album_artist_mtm::Migration),
        ]
    }
}