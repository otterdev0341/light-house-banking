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
                    .table(AssetType::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AssetType::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                    )
                    .col(
                        string(AssetType::Name)
                            .unique_key())
                    .col(
                        ColumnDef::new(AssetType::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(AssetType::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(AssetType::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_asset_type_user")
                        .from(AssetType::Table, AssetType::UserId)
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
            .drop_table(Table::drop().table(AssetType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum AssetType {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    UserId
}
