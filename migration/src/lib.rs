mod m20221218_000001_create_trade_table;
mod m20221227_000002_create_lot_table;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221218_000001_create_trade_table::Migration),
            Box::new(m20221227_000002_create_lot_table::Migration),
        ]
    }
}
