//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "asset")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Binary(16)")]
    pub id: Vec<u8>,
    pub name: String,
    #[sea_orm(column_type = "Binary(16)")]
    pub asset_type_id: Vec<u8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "Binary(16)")]
    pub user_id: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::asset_type::Entity",
        from = "Column::AssetTypeId",
        to = "super::asset_type::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    AssetType,
    #[sea_orm(has_many = "super::current_sheet::Entity")]
    CurrentSheet,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    User,
}

impl Related<super::asset_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssetType.def()
    }
}

impl Related<super::current_sheet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurrentSheet.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
