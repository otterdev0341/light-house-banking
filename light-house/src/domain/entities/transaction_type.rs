//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transaction_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Binary(16)")]
    pub id: Vec<u8>,
    #[sea_orm(unique)]
    pub name: String,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::transaction::Entity")]
    Transaction,
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
