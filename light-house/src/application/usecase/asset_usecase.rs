use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::asset_usecase::AssetUsecase, domain::{dto::asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto, ResEntryAssetDto, ResListAssetDto}, req_repository::asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}}, soc::soc_usecase::UsecaseError};






pub struct AssetUseCase<T>
where 
    T: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
{
    asset_repository: Arc<T>
}

impl<T> AssetUseCase<T>
where 
    T: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
{
    pub fn new(asset_repository: Arc<T>) -> Self {
        Self { asset_repository }
    }
}


#[async_trait::async_trait]
impl<T> AssetUsecase for AssetUseCase<T>
where 
    T: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
{
    async fn create_asset(
        &self, 
        user_id: Uuid, 
        asset_dto: ReqCreateAssetDto
    ) -> Result<ResEntryAssetDto, UsecaseError> {
        // Step 1: Create the asset in the database
        let asset_created = match self.asset_repository.create(user_id, asset_dto).await {
            Ok(asset) => asset,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Map the result to ResEntryAssetDto
        let res_entry = ResEntryAssetDto {
            id: match Uuid::from_slice(&asset_created.id) {
                Ok(id) => id.to_string(),
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
            },
            name: asset_created.name,
            asset_type: String::from("Unknown"), // Default value for asset type
            created_at: match asset_created.created_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
            updated_at: match asset_created.updated_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
        };

        // Step 3: Return the response object
        Ok(res_entry)
        
    }
    
    async fn get_asset(
        &self, 
        user_id: Uuid, 
        asset_id: Uuid
    ) -> Result<Option<ResEntryAssetDto>, UsecaseError> {
        // Step 1: Fetch the asset by user_id and asset_id from the repository
        let asset = match self.asset_repository.find_by_id(user_id, asset_id).await {
            Ok(Some(asset)) => asset,
            Ok(None) => return Ok(None), // Asset not found
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Extract the asset_type_id and convert it to a Uuid
        let asset_type_id = match Uuid::from_slice(&asset.asset_type_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UUID error
        };

        // Step 3: Fetch the asset type name using the asset_type_id
        let asset_type = match self.asset_repository.find_by_id(user_id, asset_type_id).await {
            Ok(Some(asset_type)) => asset_type.name, // If found, use the asset type name
            Ok(None) => String::from("Unknown"),    // Default value if asset type is not found
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 4: Map the asset details to ResEntryAssetDto
        let res_entry = ResEntryAssetDto {
            id: match Uuid::from_slice(&asset.id) {
                Ok(id) => id.to_string(), // Convert the asset ID from Vec<u8> to String
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UTF-8 error
            },
            name: asset.name, // Asset name
            asset_type,       // Asset type name
            created_at: match asset.created_at {
                Some(dt) => dt.to_string(), // Convert created_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
            updated_at: match asset.updated_at {
                Some(dt) => dt.to_string(), // Convert updated_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
        };

        // Step 5: Return the mapped asset details
        Ok(Some(res_entry))
    }

    async fn update_asset(
        &self, user_id: Uuid,  
        asset_id: Uuid, 
        asset_dto: ReqUpdateAssetDto
    )
         -> Result<ResEntryAssetDto, UsecaseError>
    {
        // Step 1: Call the repository to update the asset
        let result = self
            .asset_repository
            .update(asset_dto, user_id, asset_id)
            .await;

        // Step 2: Check if the result is Ok or Err
        match result {
            Ok(updated_asset) => {
                // Step 3: Map the result to ResEntryAssetDto
                let id = match Uuid::from_slice(&updated_asset.id) {
                    Ok(id) => id.to_string(),
                    Err(err) => return Err(UsecaseError::InvalidData(err.to_string())),
                };

                let asset_type_id = match Uuid::from_slice(&updated_asset.asset_type_id) {
                    Ok(id) => id,
                    Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
                };

                // Fetch the asset type name using the asset_type_id
                let asset_type = match self.asset_repository.find_by_id( user_id, asset_type_id).await {
                    Ok(Some(asset_type)) => asset_type.name,
                    Ok(None) => String::from("Unknown"), // Default value if asset type is not found
                    Err(err) => return Err(UsecaseError::from(err)),
                };

                let created_at = match updated_asset.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let updated_at = match updated_asset.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let res_entry = ResEntryAssetDto {
                    id,
                    name: updated_asset.name,
                    asset_type,
                    created_at,
                    updated_at,
                };

                // Step 4: Return the response object
                Ok(res_entry)
            },
            Err(err) => {
                // Step 6: Handle the error and return it as UsecaseError
                Err(UsecaseError::from(err))
            }
        }
    }

    async fn delete_asset(
        &self, user_id: Uuid , 
        asset_id: Uuid
    ) 
        -> Result<(), UsecaseError>
    {
        // Step 1: Check if the asset exists
        let asset_exists = match self.asset_repository.find_by_id(user_id, asset_id).await {
            Ok(Some(_)) => true, // Asset exists
            Ok(None) => return Err(UsecaseError::ResourceNotFound(format!(
                "Asset with ID '{}' not found",
                asset_id
            ))), // Asset not found
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Delete the asset if it exists
        if asset_exists {
            match self.asset_repository.delete(user_id, asset_id).await {
                Ok(_) => Ok(()), // Successfully deleted
                Err(err) => Err(UsecaseError::from(err)), // Handle repository errors
            }
        } else {
            Err(UsecaseError::ResourceNotFound(format!(
                "Asset with ID '{}' not found",
                asset_id
            )))
        }
    }

    async fn get_all_asset(
        &self, 
        user_id: Uuid
    ) -> Result<ResListAssetDto, UsecaseError> {
        // Step 1: Fetch all assets for the user from the repository
        let assets = match self.asset_repository.find_all_by_user_id(user_id).await {
            Ok(assets) => assets,
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Map the assets to ResEntryAssetDto
        let mut data = Vec::new();
        for asset in assets {
            // Extract the asset_type_id and convert it to a Uuid
            let asset_type_id = match Uuid::from_slice(&asset.asset_type_id) {
                Ok(id) => id,
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UUID error
            };

            // Fetch the asset type name using the asset_type_id
            let asset_type = match self.asset_repository.find_by_id(user_id, asset_type_id).await {
                Ok(Some(asset_type)) => asset_type.name,
                Ok(None) => String::from("Unknown"), // Default value if asset type is not found
                Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
            };

            // Map the asset to ResEntryAssetDto
            let entry = ResEntryAssetDto {
                id: match Uuid::from_slice(&asset.id) {
                    Ok(id) => id.to_string(),
                    Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UUID error
                },
                name: asset.name,
                asset_type,
                created_at: match asset.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
                updated_at: match asset.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
            };

            data.push(entry);
        }

        // Step 3: Create the response object
        let res_list = ResListAssetDto {
            length: data.len() as i32,
            data,
        };

        // Step 4: Return the response object
        Ok(res_list)
    }
}