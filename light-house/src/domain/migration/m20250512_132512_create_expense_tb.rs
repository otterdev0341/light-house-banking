use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250512_114434_create_user_tb::User, m20250512_132030_create_expense_type_tb::ExpenseType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Expense::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Expense::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )
                    .col(string(Expense::Description).not_null())
                    .col(
                        ColumnDef::new(Expense::ExpenseTypeId)
                            .uuid()
                            .not_null()
                    ).foreign_key(ForeignKey::create()
                        .name("fk_expense_expense_type")
                        .from(Expense::Table, Expense::ExpenseTypeId)
                        .to(ExpenseType::Table, ExpenseType::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(Expense::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Expense::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Expense::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_expense_user")
                        .from(Expense::Table, Expense::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(Expense::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum Expense {
    Table,
    Id,
    Description,
    ExpenseTypeId,
    CreatedAt,
    UpdatedAt,
    UserId,
}
