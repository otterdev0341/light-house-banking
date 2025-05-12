use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(TransactionType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TransactionType::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(TransactionType::Name).not_null().unique_key())
                    .col(
                        ColumnDef::new(TransactionType::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(TransactionType::UpdatedAt)
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
            .drop_table(Table::drop().table(TransactionType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
pub enum TransactionType {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
