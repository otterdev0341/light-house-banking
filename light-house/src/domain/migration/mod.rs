pub use sea_orm_migration::prelude::*;

mod m20220101_000001_asset_type_tb;
mod m20250512_114044_create_gender_tb;
mod m20250512_114434_create_user_tb;
mod m20250512_130448_create_user_role_tb;
mod m20250512_131052_create_asset_type_tb;
mod m20250512_131405_create_asset_tb;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_asset_type_tb::Migration),
            Box::new(m20250512_114044_create_gender_tb::Migration),
            Box::new(m20250512_114434_create_user_tb::Migration),
            Box::new(m20250512_130448_create_user_role_tb::Migration),
            Box::new(m20250512_131052_create_asset_type_tb::Migration),
            Box::new(m20250512_131405_create_asset_tb::Migration),
        ]
    }
}
