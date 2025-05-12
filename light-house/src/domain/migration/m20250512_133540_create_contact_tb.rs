use sea_orm_migration::{prelude::*, schema::*};

use super::m20250512_133102_create_contact_type_tb::ContactType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contact::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Contact::Name).not_null())
                    .col(string(Contact::BusinessName).not_null())
                    .col(string(Contact::Phone).not_null())
                    .col(string(Contact::Description).not_null())
                    .col(
                        ColumnDef::new(Contact::ContactTypeId)
                            .uuid()
                            .not_null(),
                    ).foreign_key(ForeignKey::create()
                        .name("fk_contact_contact_type")
                        .from(Contact::Table, Contact::ContactTypeId)
                        .to(ContactType::Table, ContactType::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(Contact::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Contact::UpdatedAt)
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
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum Contact {
    Table,
    Id,
    Name,
    BusinessName,
    Phone,
    Description,
    ContactTypeId,
    CreatedAt,
    UpdatedAt,
}
