use sea_orm_migration::{prelude::*, schema::*};

use super::m20250512_114434_create_user_tb::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(ExpenseType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExpenseType::Id)
                            .uuid()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(ExpenseType::Name).not_null())
                    .col(
                        ColumnDef::new(ExpenseType::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(ExpenseType::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(ExpenseType::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_expense_type_user")
                        .from(ExpenseType::Table, ExpenseType::UserId)
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
            .drop_table(Table::drop().table(ExpenseType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum ExpenseType {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    UserId
}
