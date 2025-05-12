use sea_orm_migration::{prelude::*, schema::*};

use super::{m20220101_000001_asset_type_tb::AssetType, m20250512_114434_create_user_tb::User};



#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Asset::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Asset::Name).not_null())
                    .col(
                        ColumnDef::new(Asset::AssetTypeId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_asset_asset_type")
                        .from(Asset::Table, Asset::AssetTypeId)
                        .to(AssetType::Table, AssetType::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(Asset::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Asset::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Asset::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_asset_user")
                        .from(Asset::Table, Asset::UserId)
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
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum Asset {
    Table,
    Id,
    Name,
    AssetTypeId,
    CreatedAt,
    UpdatedAt,
    UserId
}
