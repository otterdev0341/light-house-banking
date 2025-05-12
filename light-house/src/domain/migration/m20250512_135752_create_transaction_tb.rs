use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250512_114434_create_user_tb::User, m20250512_131405_create_asset_tb::Asset, m20250512_132512_create_expense_tb::Expense, m20250512_133540_create_contact_tb::Contact, m20250512_134954_create_transaction_type_tb::TransactionType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Transaction::TransactionTypeId)
                            .uuid()
                            .not_null(),
                    ).foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_transaction_type")
                            .from(Transaction::Table, Transaction::TransactionTypeId)
                            .to(TransactionType::Table, TransactionType::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(double(Transaction::Amount).not_null())
                    .col(
                        ColumnDef::new(Transaction::AssetId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_asset")
                            .from(Transaction::Table, Transaction::AssetId)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Transaction::DestinationAssetId)
                            .uuid()
                            .null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_destination_asset")
                            .from(Transaction::Table, Transaction::DestinationAssetId)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Transaction::ExpenseId)
                            .uuid()
                            .null(),
                    ).foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_expense")
                            .from(Transaction::Table, Transaction::ExpenseId)
                            .to(Expense::Table, Expense::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Transaction::ContactId)
                            .uuid()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_contact")
                            .from(Transaction::Table, Transaction::ContactId)
                            .to(Contact::Table, Contact::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Transaction::Note).not_null())
                    .col(
                        ColumnDef::new(Transaction::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Transaction::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Transaction::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_user")
                            .from(Transaction::Table, Transaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transaction {
    Table,
    Id,
    TransactionTypeId,
    Amount,
    AssetId,
    DestinationAssetId,
    ExpenseId,
    ContactId,
    Note,
    CreatedAt,
    UpdatedAt,
    UserId,
}
