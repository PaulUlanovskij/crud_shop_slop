pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220101_000002_insert_mock_data;
mod m20220101_000003_change_time;
mod m20220101_000004_change_order_time;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_insert_mock_data::Migration),
            Box::new(m20220101_000003_change_time::Migration),
            Box::new(m20220101_000004_change_order_time::Migration)
        ]
    }
}
