use std::sync::Arc;
use uuid::Uuid;

use crate::{application::usecase_req_impl::asset_type_usecase::AssetTypeUsecase, domain::{dto::assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto, ResEntryAssetTypeDto, ResListAssestTypeDto}, req_repository::asset_type_repository::{AssetTypeRepositoryBase, AssetTypeRepositoryUtility}}, soc::{soc_repository::RepositoryError, soc_usecase::UsecaseError}};





pub struct AssetTypeUseCase<T>
where 
    T: AssetTypeRepositoryBase + AssetTypeRepositoryUtility + Send + Sync,
{
    asset_type_repository: Arc<T>
}

impl<T> AssetTypeUseCase<T>
where 
    T: AssetTypeRepositoryBase + AssetTypeRepositoryUtility + Send + Sync,
{
    pub fn new(asset_type_repository: Arc<T>) -> Self {
        Self { asset_type_repository }
    }
}

#[async_trait::async_trait]
impl<T> AssetTypeUsecase for AssetTypeUseCase<T>
where 
    T: AssetTypeRepositoryBase + AssetTypeRepositoryUtility + Send + Sync,
{
    async fn create_asset_type(
        &self, 
        user_id: Uuid, 
        asset_type_dto: ReqCreateAssetTypeDto
    ) 
        -> Result<ResEntryAssetTypeDto, UsecaseError>
    {
        // Step 1: Call the repository to create the asset type
        let result = self.asset_type_repository.create(user_id, asset_type_dto).await;

        // Step 2: Check if the result is Ok or Err
        match result {
            Ok(asset_type) => {
                // Step 3: Map the result to ResEntryAssetTypeDto
                let id = match String::from_utf8(asset_type.id) {
                    Ok(id) => id,
                    Err(err) => return Err(UsecaseError::InvalidData(err.to_string())),
                };

                let created_at = match asset_type.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let updated_at = match asset_type.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let res_entry = ResEntryAssetTypeDto {
                    id,
                    name: asset_type.name,
                    created_at,
                    updated_at,
                };

                // Step 4: Return the response object
                Ok(res_entry)
            }
            Err(err) => {
                // Step 5: Handle the error and return it as UsecaseError
                Err(UsecaseError::from(err))
            }
        }
       
        
    }

    async fn get_asset_type(
        &self, 
        user_id: Uuid ,
        asset_type_id: Uuid
    ) 
        -> Result<Option<ResEntryAssetTypeDto>, UsecaseError>
    {
        // Step 1: Call the repository to fetch the asset type
        let result = self.asset_type_repository.find_by_id(user_id, asset_type_id).await;

        // Step 2: Check if the result is Ok or Err
        match result {
            Ok(Some(asset_type)) => {
                // Step 3: Map the result to ResEntryAssetTypeDto
                let id = match String::from_utf8(asset_type.id) {
                    Ok(id) => id,
                    Err(err) => return Err(UsecaseError::InvalidData(err.to_string())),
                };

                let created_at = match asset_type.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let updated_at = match asset_type.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let res_entry = ResEntryAssetTypeDto {
                    id,
                    name: asset_type.name,
                    created_at,
                    updated_at,
                };

                // Step 4: Return the response object wrapped in Some
                Ok(Some(res_entry))
            }
            Ok(None) => {
                // Step 5: Return None if the asset type is not found
                Ok(None)
            }
            Err(err) => {
                // Step 6: Handle the error and return it as UsecaseError
                Err(UsecaseError::from(err))
            }
        }
    }

    async fn update_asset_type(
        &self, 
        user_id: Uuid, 
        asset_type_id: Uuid, 
        asset_type_dto: ReqUpdateAssestTypeDto
    ) 
        -> Result<ResEntryAssetTypeDto, UsecaseError>
    {
        // Step 1: Call the repository to update the asset type
        let result = self
            .asset_type_repository
            .update(asset_type_dto, asset_type_id, user_id )
            .await;

        // Step 2: Check if the result is Ok or Err
        match result {
            Ok(updated_asset_type) => {
                // Step 3: Map the result to ResEntryAssetTypeDto
                let id = match String::from_utf8(updated_asset_type.id) {
                    Ok(id) => id,
                    Err(err) => return Err(UsecaseError::InvalidData(err.to_string())),
                };

                let created_at = match updated_asset_type.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let updated_at = match updated_asset_type.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                };

                let res_entry = ResEntryAssetTypeDto {
                    id,
                    name: updated_asset_type.name,
                    created_at,
                    updated_at,
                };

                // Step 4: Return the response object
                Ok(res_entry)
            },
            Err(RepositoryError::NotFound(_)) => {
                // Step 5: Handle the case where the asset type is not found
                Err(UsecaseError::Unexpected("Asset type not found".to_string()))
            }
            Err(err) => {
                // Step 6: Handle the error and return it as UsecaseError
                Err(UsecaseError::from(err))
            }
            
        }
    }

    async fn delete_asset_type(
        &self, 
        user_id: Uuid ,
        asset_type_id: Uuid
    ) 
        -> Result<(), UsecaseError>
    {
        let result = self.asset_type_repository.delete(user_id, asset_type_id).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(UsecaseError::from(err)),
        }
    }

    async fn get_all_asset_types(
        &self, 
        user_id: Uuid
    ) 
        -> Result<ResListAssestTypeDto, UsecaseError>
    {
        let result = self.asset_type_repository.find_all_by_user_id(user_id).await;
        match result {
            Ok(asset_types) => {
                let mut res_list = Vec::new();
                for asset_type in asset_types {
                    let id = match String::from_utf8(asset_type.id) {
                        Ok(id) => id,
                        Err(err) => return Err(UsecaseError::InvalidData(err.to_string())),
                    };

                    let created_at = match asset_type.created_at {
                        Some(dt) => dt.to_string(),
                        None => String::from(""),
                    };

                    let updated_at = match asset_type.updated_at {
                        Some(dt) => dt.to_string(),
                        None => String::from(""),
                    };

                    let res_entry = ResEntryAssetTypeDto {
                        id,
                        name: asset_type.name,
                        created_at,
                        updated_at,
                    };
                    res_list.push(res_entry);
                }

                // Step 4: Return the response object
                Ok(ResListAssestTypeDto { length: res_list.len() as i32, data: res_list })
            }
            Err(err) => Err(UsecaseError::from(err)),
        }

    }
}