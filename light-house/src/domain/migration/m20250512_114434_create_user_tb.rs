

use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250512_114044_create_gender_tb::Gender, m20250512_130448_create_user_role_tb::UserRole};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(User::Username).not_null().unique_key())
                    .col(string(User::Password).not_null())
                    .col(string(User::Email).not_null().unique_key())
                    .col(string(User::FirstName).not_null())
                    .col(string(User::LastName).not_null())
                    .col(string(User::McpToken).unique_key())
                    .col(
                        ColumnDef::new(User::GenderId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_gender")
                        .from(User::Table, User::GenderId)
                        .to(Gender::Table, Gender::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(User::UserRoleId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_user_role")
                        .from(User::Table, User::UserRoleId)
                        .to(UserRole::Table, UserRole::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
                )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Email,
    FirstName,
    LastName,
    McpToken,
    GenderId,
    UserRoleId,
    CreatedAt,
    UpdatedAt,
}
