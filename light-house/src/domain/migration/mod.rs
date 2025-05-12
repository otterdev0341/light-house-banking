pub use sea_orm_migration::prelude::*;

mod m20220101_000001_asset_type_tb;
mod m20250512_114044_create_gender_tb;
mod m20250512_114434_create_user_tb;
mod m20250512_130448_create_user_role_tb;
mod m20250512_131405_create_asset_tb;
mod m20250512_132030_create_expense_type_tb;
mod m20250512_132512_create_expense_tb;
mod m20250512_133102_create_contact_type_tb;
mod m20250512_133540_create_contact_tb;
mod m20250512_134954_create_transaction_type_tb;
mod m20250512_135752_create_transaction_tb;
mod m20250512_142622_create_current_sheet;
mod m20250512_143438_create_user_contact;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 1st teir
            Box::new(m20250512_114044_create_gender_tb::Migration),
            Box::new(m20250512_130448_create_user_role_tb::Migration),
            Box::new(m20250512_114434_create_user_tb::Migration),
            Box::new(m20220101_000001_asset_type_tb::Migration),
            Box::new(m20250512_131405_create_asset_tb::Migration),
            Box::new(m20250512_132030_create_expense_type_tb::Migration),
            Box::new(m20250512_133102_create_contact_type_tb::Migration),
            Box::new(m20250512_134954_create_transaction_type_tb::Migration),
            // 2nd teir
            Box::new(m20250512_132512_create_expense_tb::Migration),
            Box::new(m20250512_133540_create_contact_tb::Migration),
            Box::new(m20250512_143438_create_user_contact::Migration),
            // 3rd teir
            Box::new(m20250512_135752_create_transaction_tb::Migration),
            Box::new(m20250512_142622_create_current_sheet::Migration),
        ]
    }
}


// // 1st teir
// Box::new(m20220101_000001_asset_type_tb::Migration),
// Box::new(m20250512_131052_create_asset_type_tb::Migration),
// Box::new(m20250512_114044_create_gender_tb::Migration),
// Box::new(m20250512_130448_create_user_role_tb::Migration),
// Box::new(m20250512_131405_create_asset_tb::Migration),
// Box::new(m20250512_132030_create_expense_type_tb::Migration),
// Box::new(m20250512_133102_create_contact_type_tb::Migration),
// Box::new(m20250512_134954_create_transaction_type_tb::Migration),
// // 2nd teir
// Box::new(m20250512_114434_create_user_tb::Migration),
// Box::new(m20250512_132512_create_expense_tb::Migration),
// Box::new(m20250512_133540_create_contact_tb::Migration),
// Box::new(m20250512_143438_create_user_contact::Migration),
// // 3rd teir
// Box::new(m20250512_135752_create_transaction_tb::Migration),
// Box::new(m20250512_142622_create_current_sheet::Migration),