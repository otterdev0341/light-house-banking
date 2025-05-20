#[allow(unused)]
use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250512_114434_create_user_tb::User, m20250512_131405_create_asset_tb::Asset, m20250512_135752_create_transaction_tb::Transaction};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        manager
            .create_table(
                Table::create()
                    .table(CurrentSheet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CurrentSheet::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CurrentSheet::AssetId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_current_sheet_asset_id")
                            .from(CurrentSheet::Table, CurrentSheet::AssetId)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(CurrentSheet::Balance)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CurrentSheet::LastTransactionId)
                            .uuid()
                            
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_current_sheet_last_transaction_id")
                            .from(CurrentSheet::Table, CurrentSheet::LastTransactionId)
                            .to(Transaction::Table, Transaction::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(CurrentSheet::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(CurrentSheet::UserId)
                        .uuid()
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_current_sheet_user_id")
                            .from(CurrentSheet::Table, CurrentSheet::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        manager
            .drop_table(Table::drop().table(CurrentSheet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum CurrentSheet {
    Table,
    Id,
    AssetId,
    Balance,
    LastTransactionId,
    UpdatedAt,
    UserId
}
