#[allow(unused)]
use sea_orm_migration::{prelude::*, schema::*};

use super::{m20250512_114434_create_user_tb::User, m20250512_133540_create_contact_tb::Contact};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(UserContact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserContact::UserId)
                            .uuid()
                            .not_null()
                            
                            
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_contact_user_id")
                            .from(UserContact::Table, UserContact::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(UserContact::ContactId)
                            .uuid()
                            .not_null()
                            
                    )
                    .primary_key(
                        Index::create()
                            .col(UserContact::UserId)
                            .col(UserContact::ContactId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_contact_contact_id")
                            .from(UserContact::Table, UserContact::ContactId)
                            .to(Contact::Table, Contact::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(UserContact::CreatedAt)
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
            .drop_table(Table::drop().table(UserContact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserContact {
    Table,
    UserId,
    ContactId,
    CreatedAt
}
