use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20221218_000001_create_trade_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create Trade table
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Trade::Table)
                    .col(
                        ColumnDef::new(Trade::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Trade::ValrId).uuid().not_null())
                    .col(
                        ColumnDef::new(Trade::CreatedTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Trade::Price).float().not_null())
                    .col(ColumnDef::new(Trade::Quantity).float().not_null())
                    .col(ColumnDef::new(Trade::CurrencyPair).text().not_null())
                    .col(
                        ColumnDef::new(Trade::TradedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Trade::TakerSide).text().not_null())
                    .col(ColumnDef::new(Trade::SequenceId).integer().not_null())
                    .col(ColumnDef::new(Trade::QuoteVolume).float().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop Trade table
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Trade::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Trade {
    Table,
    Id,
    ValrId,
    CreatedTime,
    Price,
    Quantity,
    CurrencyPair,
    TradedAt,
    TakerSide,
    SequenceId,
    QuoteVolume,
}
