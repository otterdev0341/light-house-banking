use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::transaction_usecase::TransferUsecase, domain::{dto::transaction_dto::{ReqCreateTransferDto, ReqUpdateTransferDto, ResEntryTransferDto, ResListTransferDto}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}, transaction_repository::{TransactionTypeRepositoryUtility, TransferRepositoryUtility}}}, soc::{soc_repository::RepositoryError, soc_usecase::UsecaseError}};





pub struct TransferUseCase<T, A, C, TT>
where
    T: TransferRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
{
    transfer_repo: Arc<T>,
    asset_repo: Arc<A>,
    contact_repo: Arc<C>,
    transaction_type_repo: Arc<TT>,
}




impl<T, A, C, TT> TransferUseCase<T, A, C, TT>
where
    T: TransferRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
{
    pub fn new(
        transfer_repo: Arc<T>,
        asset_repo: Arc<A>,
        contact_repo: Arc<C>,
        transaction_type_repo: Arc<TT>,
    ) -> Self {
        Self {
            transfer_repo,
            asset_repo,
            contact_repo,
            transaction_type_repo,
        }
    }
}



#[async_trait::async_trait]
impl<T, A, C, TT> TransferUsecase for TransferUseCase<T, A, C, TT>
where
    T: TransferRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
{


    async fn create_transfer(&self, user_id: Uuid, transfer_dto: ReqCreateTransferDto) -> Result<ResEntryTransferDto, UsecaseError>
    {
       // Step 1: Create the transfer in the database
       let transfer_created = match self.transfer_repo.create_transfer(user_id, transfer_dto).await {
        Ok(transfer) => transfer,
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 2: Fetch the transaction type name using the transaction_type_id
    let transaction_type_name = match self
        .transaction_type_repo
        .get_transaction_type_by_id(user_id, Uuid::from_slice(&transfer_created.transaction_type_id).map_err(|e| UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string()))))?)
        .await
    {
        Ok(Some(transaction_type)) => transaction_type.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 3: Fetch the asset name using the asset_id
    let asset_name = match self
        .asset_repo
        .find_by_id(user_id, Uuid::from_slice(&transfer_created.asset_id).map_err(|e| UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string()))))?)
        .await
    {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 4: Fetch the destination asset name using the destination_asset_id
    let destination_asset_name = match self
        .asset_repo
        .find_by_id(
            user_id,
            Uuid::from_slice(
                transfer_created
                    .destination_asset_id
                    .as_deref()
                    .ok_or_else(|| UsecaseError::from(RepositoryError::InvalidInput("fail to convert uuid".to_string())))?,
            )
            .map_err(|e| UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string()))))?,
        )
        .await
    {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 5: Fetch the contact name using the contact_id
    let contact_name = match self
        .contact_repo
        .find_by_user_id_and_contact_id(
            user_id,
            Uuid::from_slice(
                transfer_created
                    .contact_id
                    .as_deref()
                    .ok_or_else(|| UsecaseError::from(RepositoryError::InvalidInput("fail to convert uuid".to_string())))?,
            )
            .map_err(|e| UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string()))))?,
        )
        .await
    {
        Ok(Some(contact)) => contact.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 6: Map the result to ResEntryTransferDto
    let res_entry = ResEntryTransferDto {
        id: match Uuid::from_slice(&transfer_created.id) {
            Ok(id) => id.to_string(),
            Err(e) => return Err(UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))),
        },
        transaction_type_name,
        amount: transfer_created.amount,
        asset_name,
        destination_asset_name,
        contact_name,
        note: transfer_created.note,
        created_at: transfer_created.created_at.map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
        updated_at: transfer_created.updated_at.map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
    };

    // Step 7: Return the response object
    Ok(res_entry)
    }

    async fn get_transfer(&self, user_id: Uuid , transaction_id: Uuid) -> Result<Option<ResEntryTransferDto>, UsecaseError>
    {
        // Step 1: Fetch the transfer by user_id and transaction_id from the repository
    let transfer = match self.transfer_repo.get_transfer_by_id(user_id, transaction_id).await {
        Ok(Some(transfer)) => {
            // Step 2: Fetch the transaction type name using the transaction_type_id
            let transaction_type_name = match self
                .transaction_type_repo
                .get_transaction_type_by_id(
                    user_id,
                    Uuid::from_slice(&transfer.transaction_type_id).map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(transaction_type)) => transaction_type.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 3: Fetch the asset name using the asset_id
            let asset_name = match self
                .asset_repo
                .find_by_id(
                    user_id,
                    Uuid::from_slice(&transfer.asset_id).map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(asset)) => asset.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 4: Fetch the destination asset name using the destination_asset_id
            let destination_asset_name = match self
                .asset_repo
                .find_by_id(
                    user_id,
                    Uuid::from_slice(
                        transfer
                            .destination_asset_id
                            .as_deref()
                            .ok_or_else(|| {
                                UsecaseError::from(RepositoryError::InvalidInput(
                                    "fail to convert uuid".to_string(),
                                ))
                            })?,
                    )
                    .map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(asset)) => asset.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 5: Fetch the contact name using the contact_id
            let contact_name = match self
                .contact_repo
                .find_by_user_id_and_contact_id(
                    user_id,
                    Uuid::from_slice(
                        transfer
                            .contact_id
                            .as_deref()
                            .ok_or_else(|| {
                                UsecaseError::from(RepositoryError::InvalidInput(
                                    "fail to convert uuid".to_string(),
                                ))
                            })?,
                    )
                    .map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(contact)) => contact.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 6: Map the result to ResEntryTransferDto
            Some(ResEntryTransferDto {
                id: match Uuid::from_slice(&transfer.id) {
                    Ok(id) => id.to_string(),
                    Err(e) => return Err(UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))),
                },
                transaction_type_name,
                amount: transfer.amount,
                asset_name,
                destination_asset_name,
                contact_name,
                note: transfer.note,
                created_at: transfer
                    .created_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
                updated_at: transfer
                    .updated_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
            })
        }
        Ok(None) => return Ok(None), // Transfer not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 7: Return the mapped transfer details
    Ok(transfer)
    }

    async fn update_transfer(&self, user_id: Uuid,  transaction_id: Uuid, transfer_dto: ReqUpdateTransferDto) -> Result<ResEntryTransferDto, UsecaseError>
    {
        // Step 1: Call the repository to update the transfer
    let updated_transfer = match self
    .transfer_repo
    .update_transfer(user_id, transaction_id, transfer_dto)
    .await
    {
        Ok(transfer) => transfer,
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 2: Fetch the transaction type name using the transaction_type_id
    let transaction_type_name = match self
        .transaction_type_repo
        .get_transaction_type_by_id(
            user_id,
            Uuid::from_slice(&updated_transfer.transaction_type_id).map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(transaction_type)) => transaction_type.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 3: Fetch the asset name using the asset_id
    let asset_name = match self
        .asset_repo
        .find_by_id(
            user_id,
            Uuid::from_slice(&updated_transfer.asset_id).map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 4: Fetch the destination asset name using the destination_asset_id
    let destination_asset_name = match self
        .asset_repo
        .find_by_id(
            user_id,
            Uuid::from_slice(
                updated_transfer
                    .destination_asset_id
                    .as_deref()
                    .ok_or_else(|| {
                        UsecaseError::from(RepositoryError::InvalidInput(
                            "fail to convert uuid".to_string(),
                        ))
                    })?,
            )
            .map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 5: Fetch the contact name using the contact_id
    let contact_name = match self
        .contact_repo
        .find_by_user_id_and_contact_id(
            user_id,
            Uuid::from_slice(
                updated_transfer
                    .contact_id
                    .as_deref()
                    .ok_or_else(|| {
                        UsecaseError::from(RepositoryError::InvalidInput(
                            "fail to convert uuid".to_string(),
                        ))
                    })?,
            )
            .map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(contact)) => contact.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 6: Map the result to ResEntryTransferDto
    let res_entry = ResEntryTransferDto {
        id: match Uuid::from_slice(&updated_transfer.id) {
            Ok(id) => id.to_string(),
            Err(e) => return Err(UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))),
        },
        transaction_type_name,
        amount: updated_transfer.amount,
        asset_name,
        destination_asset_name,
        contact_name,
        note: updated_transfer.note,
        created_at: updated_transfer
            .created_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
        updated_at: updated_transfer
            .updated_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
    };

        // Step 7: Return the response object
    Ok(res_entry)
    }

    async fn delete_transfer(&self, user_id: Uuid , transaction_id: Uuid) -> Result<(), UsecaseError>
    {
        // Step 1: Check if the transfer exists
    let transfer_exists = match self.transfer_repo.get_transfer_by_id(user_id, transaction_id).await {
        Ok(Some(_)) => true, // Transfer exists
        Ok(None) => {
            return Err(UsecaseError::ResourceNotFound(format!(
                "Transfer with ID '{}' not found",
                transaction_id
            )))
        } // Transfer not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Delete the transfer if it exists
    if transfer_exists {
        match self.transfer_repo.delete_transfer(user_id, transaction_id).await {
            Ok(_) => Ok(()), // Successfully deleted
            Err(err) => Err(UsecaseError::from(err)), // Handle repository errors
        }
    } else {
        Err(UsecaseError::ResourceNotFound(format!(
            "Transfer with ID '{}' not found",
            transaction_id
        )))
    }
    }

    async fn get_all_transfer(&self, user_id: Uuid) -> Result<ResListTransferDto, UsecaseError>
    {
        // Step 1: Fetch all transfers for the user from the repository
    let transfers = match self.transfer_repo.get_all_transfers_by_user(user_id).await {
        Ok(transfers) => transfers,
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Map the transfers to ResEntryTransferDto
    let mut data = Vec::new();
    for transfer in transfers {
        // Fetch the transaction type name using the transaction_type_id
        let transaction_type_name = match self
            .transaction_type_repo
            .get_transaction_type_by_id(
                user_id,
                Uuid::from_slice(&transfer.transaction_type_id).map_err(|e| {
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(transaction_type)) => transaction_type.name,
            Ok(None) => String::from("Unknown"),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Fetch the asset name using the asset_id
        let asset_name = match self
            .asset_repo
            .find_by_id(
                user_id,
                Uuid::from_slice(&transfer.asset_id).map_err(|e| {
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(asset)) => asset.name,
            Ok(None) => String::from("Unknown"),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Fetch the destination asset name using the destination_asset_id
        let destination_asset_name = match self
            .asset_repo
            .find_by_id(
                user_id,
                Uuid::from_slice(
                    transfer
                        .destination_asset_id
                        .as_deref()
                        .ok_or_else(|| {
                            UsecaseError::from(RepositoryError::InvalidInput(
                                "fail to convert uuid".to_string(),
                            ))
                        })?,
                )
                .map_err(|e| {
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(asset)) => asset.name,
            Ok(None) => String::from("Unknown"),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Fetch the contact name using the contact_id
        let contact_name = match self
            .contact_repo
            .find_by_user_id_and_contact_id(
                user_id,
                Uuid::from_slice(
                    transfer
                        .contact_id
                        .as_deref()
                        .ok_or_else(|| {
                            UsecaseError::from(RepositoryError::InvalidInput(
                                "fail to convert uuid".to_string(),
                            ))
                        })?,
                )
                .map_err(|e| {
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(contact)) => contact.name,
            Ok(None) => String::from("Unknown"),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Map the transfer to ResEntryTransferDto
        let res_entry = ResEntryTransferDto {
            id: match Uuid::from_slice(&transfer.id) {
                Ok(id) => id.to_string(),
                Err(e) => return Err(UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))),
            },
            transaction_type_name,
            amount: transfer.amount,
            asset_name,
            destination_asset_name,
            contact_name,
            note: transfer.note,
            created_at: transfer
                .created_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
            updated_at: transfer
                .updated_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
        };

        data.push(res_entry);
    }

        // Step 3: Create the response object
        let res_list = ResListTransferDto {
            length: data.len() as i64,
            data,
        };

        // Step 4: Return the response object
        Ok(res_list)
    }
}