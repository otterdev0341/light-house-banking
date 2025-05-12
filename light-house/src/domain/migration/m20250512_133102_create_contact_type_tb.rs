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
                    .table(ContactType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactType::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(ContactType::Name).not_null())
                    .col(
                        ColumnDef::new(ContactType::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(ContactType::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(ContactType::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_contact_type_user")
                        .from(ContactType::Table, ContactType::UserId)
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
            .drop_table(Table::drop().table(ContactType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum ContactType {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    UserId,
}
