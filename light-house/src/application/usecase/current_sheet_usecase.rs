use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::prelude::ToPrimitive;
use crate::{application::usecase_req_impl::current_sheet_usecase::CurrentSheetUsecase, domain::{dto::current_sheet_dto::{ResCurrentSheetDto, ResListCurrentSheetDto}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, asset_type_repository::{AssetTypeRepositoryBase, AssetTypeRepositoryUtility}, balance_repository::{BalanceRepositoryBase, BalanceRepositoryUtill}}}, soc::{soc_repository::RepositoryError, soc_usecase::UsecaseError}};






pub struct CurrentUseCase<T, A, AT>
where 
    T: BalanceRepositoryBase + BalanceRepositoryUtill + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    AT: AssetTypeRepositoryBase + AssetTypeRepositoryUtility + Send + Sync,
{
    balance_repo: Arc<T>,
    asset_repo: Arc<A>,
    asset_type_repo: Arc<AT>,
}


impl<T, A, AT> CurrentUseCase<T, A, AT>
where 
    T: BalanceRepositoryBase + BalanceRepositoryUtill + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    AT: AssetTypeRepositoryBase + AssetTypeRepositoryUtility + Send + Sync,
{
    pub fn new(balance_repo: Arc<T>, asset_repo: Arc<A>, asset_type_repo: Arc<AT>) -> Self {
        Self { balance_repo, asset_repo, asset_type_repo }
    }
}


#[async_trait::async_trait]
impl<T,A, AT > CurrentSheetUsecase for CurrentUseCase<T, A, AT>
where 
    T: BalanceRepositoryBase + BalanceRepositoryUtill + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    AT: AssetTypeRepositoryBase + AssetTypeRepositoryUtility + Send + Sync,
{
    async fn get_current_sheet_by_id(&self, user_id: Uuid, current_sheet_id: Uuid) -> Result<Option<ResCurrentSheetDto>, UsecaseError> {
         // Step 1: Fetch the current sheet record by user_id and current_sheet_id from the balance repository
    let current_sheet = match self
    .balance_repo
    .get_current_sheet_by_asset_id(user_id, current_sheet_id)
    .await
    {
        Ok(Some(sheet)) => sheet,
        Ok(None) => return Ok(None), // Current sheet not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Fetch the asset name using the asset_id
    let asset_name = match self
        .asset_repo
        .find_by_id(
            user_id,
            Uuid::from_slice(&current_sheet.asset_id).map_err(|e| {
                UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
            })?,
        )
        .await
    {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 3: Map the result to ResCurrentSheetDto
    let res_current_sheet = ResCurrentSheetDto {
        id: String::from_utf8(current_sheet.id).map_err(|e| {
            UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
        })?,
        asset_name,
        balance: current_sheet.balance.to_f64().ok_or_else(|| {
            UsecaseError::from(RepositoryError::InvalidInput("Failed to convert Decimal to f64".to_string()))
        })?,
        last_transaction_id: current_sheet
            .last_transaction_id
            .map(|id| String::from_utf8(id).unwrap_or_else(|_| "Invalid ID".to_string())),
        updated_at: current_sheet
            .updated_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
    };

    // Step 4: Return the response object
    Ok(Some(res_current_sheet))
    }

    async fn get_all_current_sheets_by_user(&self, user_id: Uuid) -> Result<ResListCurrentSheetDto, UsecaseError> {
        // Step 1: Fetch all current sheet records for the user from the balance repository
    let current_sheets = match self.balance_repo.get_all_current_sheets_by_user(user_id).await {
        Ok(sheets) => sheets,
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Map the current sheet records to ResCurrentSheetDto
    let mut data = Vec::new();
    for sheet in current_sheets {
        // Fetch the asset name using the asset_id
        let asset_name = match self
            .asset_repo
            .find_by_id(
                user_id,
                Uuid::from_slice(&sheet.asset_id).map_err(|e| {
                    UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
                })?,
            )
            .await
        {
            Ok(Some(asset)) => asset.name,
            Ok(None) => String::from("Unknown"),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Map the current sheet record to ResCurrentSheetDto
        let res_entry = ResCurrentSheetDto {
            id: String::from_utf8(sheet.id).map_err(|e| {
                UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
            })?,
            asset_name,
            balance: sheet.balance.to_f64().ok_or_else(|| {
                UsecaseError::from(RepositoryError::InvalidInput(
                    "Failed to convert Decimal to f64".to_string(),
                ))
            })?,
            last_transaction_id: sheet
                .last_transaction_id
                .map(|id| String::from_utf8(id).unwrap_or_else(|_| "Invalid ID".to_string())),
            updated_at: sheet
                .updated_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
        };

        data.push(res_entry);
    }

        // Step 3: Create the response object
        let res_list = ResListCurrentSheetDto {
            length: data.len() as i32,
            data,
        };

        // Step 4: Return the response object
        Ok(res_list)
    }



    async fn get_all_current_sheets_by_asset_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<ResListCurrentSheetDto, UsecaseError> {
        // Step 1: Fetch all asset IDs associated with the given asset_type_id
    let asset_ids = match self
    .asset_repo
    .find_by_id(user_id, asset_id)
    .await
    {
        Ok(assets) => assets.into_iter().map(|asset| {
            Uuid::from_slice(&asset.id).map_err(|e| {
                UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
            })
        }).collect::<Result<Vec<_>, _>>()?,
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Fetch all current sheet records for the fetched asset IDs
    let current_sheets = match self
        .balance_repo
        .get_all_current_sheets_by_asset_id(user_id, asset_ids.into_iter().next().ok_or_else(|| {
            UsecaseError::from(RepositoryError::InvalidInput("No asset IDs provided".to_string()))
        })?)
        .await
        {
            Ok(sheets) => sheets,
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 3: Map the current sheet records to ResCurrentSheetDto
        let mut data = Vec::new();
        for sheet in current_sheets {
            // Fetch the asset name using the asset_id
            let asset_name = match self
                .asset_repo
                .find_by_id(
                    user_id,
                    Uuid::from_slice(&sheet.asset_id).map_err(|e| {
                        UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
                    })?,
                )
                .await
            {
                Ok(Some(asset)) => asset.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Map the current sheet record to ResCurrentSheetDto
            let res_entry = ResCurrentSheetDto {
                id: String::from_utf8(sheet.id).map_err(|e| {
                    UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
                })?,
                asset_name,
                balance: sheet.balance.to_f64().ok_or_else(|| {
                    UsecaseError::from(RepositoryError::InvalidInput(
                        "Failed to convert Decimal to f64".to_string(),
                    ))
                })?,
                last_transaction_id: sheet
                    .last_transaction_id
                    .map(|id| String::from_utf8(id).unwrap_or_else(|_| "Invalid ID".to_string())),
                updated_at: sheet
                    .updated_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
            };

            data.push(res_entry);
        }

        // Step 4: Create the response object
        let res_list = ResListCurrentSheetDto {
            length: data.len() as i32,
            data,
        };

        // Step 5: Return the response object
        Ok(res_list)
    }
}