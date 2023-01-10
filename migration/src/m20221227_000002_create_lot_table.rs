use super::m20221218_000001_create_trade_table::Trade;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20221227_000002_create_lot_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create Trade table
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Lot::Table)
                    .col(
                        ColumnDef::new(Lot::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Lot::ValrId).uuid().not_null())
                    .col(
                        ColumnDef::new(Lot::CreatedTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Lot::LotPrice).float().not_null())
                    .col(ColumnDef::new(Lot::Price).float().not_null())
                    .col(ColumnDef::new(Lot::Quantity).float().not_null())
                    .col(ColumnDef::new(Lot::CurrencyPair).text().not_null())
                    .col(ColumnDef::new(Lot::PostOnly).boolean().not_null())
                    .col(ColumnDef::new(Lot::Side).text().not_null())
                    .col(ColumnDef::new(Lot::CustomerOrderId).text().not_null())
                    .col(ColumnDef::new(Lot::TimeInForce).text().not_null())
                    .col(ColumnDef::new(Lot::LotStatus).text().not_null())
                    .col(ColumnDef::new(Lot::ProfitTotal).float().not_null())
                    .col(ColumnDef::new(Lot::AmountOfTrades).integer().not_null())
                    .col(ColumnDef::new(Lot::BatchId).integer().not_null())
                    .col(ColumnDef::new(Lot::FeeCurrencyZar).float().not_null())
                    .col(ColumnDef::new(Lot::FeeCurrencyCrypto).float().not_null())
                    .col(ColumnDef::new(Lot::TradeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lot-trade_id")
                            .from(Lot::Table, Lot::TradeId)
                            .to(Trade::Table, Trade::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop Trade table
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lot::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Lot {
    Table,
    Id,
    ValrId,
    CreatedTime,
    LotPrice,
    Price,
    Quantity,
    CurrencyPair,
    PostOnly,
    Side,
    CustomerOrderId,
    TimeInForce,
    LotStatus,
    ProfitTotal,
    AmountOfTrades,
    BatchId,
    FeeCurrencyZar,
    FeeCurrencyCrypto,
    TradeId,
}
