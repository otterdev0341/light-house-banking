use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{domain::{entities::gender, req_repository::gender_repository::GenderRepository}, soc::soc_repository::RepositoryError};




pub struct GenderRepositoryImpl {
    pub db_pool: Arc<DatabaseConnection>
}


impl GenderRepositoryImpl {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self { db_pool }
    }
}


#[async_trait::async_trait] 
impl GenderRepository for GenderRepositoryImpl {

    async fn get_gender_by_id(&self, gender_id: Uuid) -> Result<Option<gender::Model>, RepositoryError>
    {
        // Query the database to find the gender by its ID
        let gender = gender::Entity::find_by_id(gender_id.as_bytes().to_vec())
            .one(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(gender)
    }

    async fn get_all_gender(&self) -> Result<Vec<gender::Model>, RepositoryError>
    {
        // Query the database to retrieve all gender records
        let genders = gender::Entity::find()
            .all(self.db_pool.as_ref())
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;

        Ok(genders)
    }
}