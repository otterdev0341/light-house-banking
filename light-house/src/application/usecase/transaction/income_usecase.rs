use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::transaction_usecase::RecordIncomeUsecase, domain::{dto::transaction_dto::{ReqCreateIncomeDto, ReqUpdateIncomeDto, ResEntryIncomeDto, ResListIncomeDto}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}, transaction_repository::{RecordIncomeRepositoryUtility, TransactionTypeRepositoryUtility}}}, soc::{soc_repository::RepositoryError, soc_usecase::UsecaseError}};






pub struct IncomeUseCase<T, A, C, TT>
where
    T: RecordIncomeRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
{
    income_repo: Arc<T>,
    asset_repo: Arc<A>,
    contact_repo: Arc<C>,
    transaction_type_repo: Arc<TT>,
}


impl<T, A, C, TT> IncomeUseCase<T, A, C, TT>
where
    T: RecordIncomeRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
{
    pub fn new(
        income_repo: Arc<T>,
        asset_repo: Arc<A>,
        contact_repo: Arc<C>,
        transaction_type_repo: Arc<TT>,
    ) -> Self {
        Self {
            income_repo,
            asset_repo,
            contact_repo,
            transaction_type_repo,
        }
    }
}


#[async_trait::async_trait]
impl<T, A, C, TT> RecordIncomeUsecase for IncomeUseCase<T, A, C, TT>
where
    T: RecordIncomeRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
{

    async fn create_income(&self, user_id: Uuid, income_dto: ReqCreateIncomeDto) -> Result<ResEntryIncomeDto, UsecaseError> {
        // Step 1: Create the income record in the database
        let income_created = match self.income_repo.create_income_record(user_id, income_dto).await {
            Ok(income) => income,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        log::debug!("Income record created: {:?}", income_created);
        
        // Step 2: Fetch the transaction type name using the transaction_type_id
        let transaction_type_name = match self
            .transaction_type_repo
            .get_transaction_type_by_id(
                user_id,
                Uuid::from_slice(&income_created.transaction_type_id).map_err(|e| {
                    log::error!("Failed to parse transaction_type_id: {}", e);
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(transaction_type)) => transaction_type.name,
            Ok(None) => {
                log::error!("Transaction type not found for ID: {:?}", income_created.transaction_type_id);
                String::from("Unknown")
            },
            Err(err) => {
                log::error!("Error fetching transaction type: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        log::debug!("Transaction type name: {}", transaction_type_name);

        // Step 3: Fetch the asset name using the asset_id
        let asset_name = match self
            .asset_repo
            .find_by_id(
                user_id,
                Uuid::from_slice(&income_created.asset_id).map_err(|e| {
                    log::error!("Failed to parse asset_id: {}", e);
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(asset)) => asset.name,
            Ok(None) => {
                log::error!("Asset not found for ID: {:?}", income_created.asset_id);
                String::from("Unknown")
            },
            Err(err) => {
                log::error!("Error fetching asset: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        log::debug!("Asset name: {}", asset_name);

        // Step 4: Fetch the contact name using the contact_id
        let contact_name = match self
            .contact_repo
            .find_by_user_id_and_contact_id(
                user_id,
                Uuid::from_slice(
                    income_created
                        .contact_id
                        .as_deref()
                        .ok_or_else(|| {
                            UsecaseError::from(RepositoryError::InvalidInput(
                                "Contact ID is missing".to_string(),
                            ))
                        })?,
                )
                .map_err(|e| {
                    log::error!("Failed to parse contact_id: {}", e);
                    UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                })?,
            )
            .await
        {
            Ok(Some(contact)) => contact.name,
            Ok(None) => {
                log::error!("Contact not found for ID: {:?}", income_created.contact_id);
                String::from("Unknown")
            },
            Err(err) => {
                log::error!("Error fetching contact: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        log::debug!("Contact name: {}", contact_name);

        // Step 5: Map the result to ResEntryIncomeDto
        let res_entry = ResEntryIncomeDto {
            id: match Uuid::from_slice(&income_created.id) {
                Ok(uuid) => uuid.to_string(),
                Err(err) => {
                    log::error!("Failed to parse income ID as UUID: {}", err);
                    return Err(UsecaseError::Unexpected("Invalid income ID".to_string()));
                }
            },
            transaction_type_name,
            amount: income_created.amount,
            asset_name,
            contact_name,
            note: income_created.note,
            created_at: income_created
                .created_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
            updated_at: income_created
                .updated_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
        };
        log::info!("Amount in IncomeUseCase: {}", res_entry.amount);
        log::debug!("Response DTO: {:?}", res_entry);

        // Step 6: Return the response object
        Ok(res_entry)
    }



    async fn get_income(&self, user_id: Uuid , transaction_id: Uuid) -> Result<Option<ResEntryIncomeDto>, UsecaseError>
    {   
        // Step 1: Fetch the income record by user_id and transaction_id from the repository
    let income = match self.income_repo.get_income_record_by_id(user_id, transaction_id).await {
        Ok(Some(income)) => {
            // Step 2: Fetch the transaction type name using the transaction_type_id
            let transaction_type_name = match self
                .transaction_type_repo
                .get_transaction_type_by_id(
                    user_id,
                    Uuid::from_slice(&income.transaction_type_id).map_err(|e| {
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
                    Uuid::from_slice(&income.asset_id).map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(asset)) => asset.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 4: Fetch the contact name using the contact_id
            let contact_name = match self
                .contact_repo
                .find_by_user_id_and_contact_id(
                    user_id,
                    Uuid::from_slice(
                        income
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

            // Step 5: Map the result to ResEntryIncomeDto
            Some(ResEntryIncomeDto {
                id: String::from_utf8(income.id).map_err(|e| {
                    UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
                })?,
                transaction_type_name,
                amount: income.amount,
                asset_name,
                contact_name,
                note: income.note,
                created_at: income
                    .created_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
                updated_at: income
                    .updated_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
            })
        }
        Ok(None) => return Ok(None), // Income record not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 6: Return the mapped income details
    Ok(income)
    }

    async fn update_income(&self, user_id: Uuid,  transaction_id: Uuid, income_dto: ReqUpdateIncomeDto) -> Result<ResEntryIncomeDto, UsecaseError>
    {   
        // Step 1: Call the repository to update the income record
    let updated_income = match self
    .income_repo
    .update_income_record(user_id, transaction_id, income_dto)
    .await
{
    Ok(income) => income,
    Err(err) => return Err(UsecaseError::from(err)),
};

// Step 2: Fetch the transaction type name using the transaction_type_id
let transaction_type_name = match self
    .transaction_type_repo
    .get_transaction_type_by_id(
        user_id,
        Uuid::from_slice(&updated_income.transaction_type_id).map_err(|e| {
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
        Uuid::from_slice(&updated_income.asset_id).map_err(|e| {
            UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
        })?,
    )
    .await
{
    Ok(Some(asset)) => asset.name,
    Ok(None) => String::from("Unknown"),
    Err(err) => return Err(UsecaseError::from(err)),
};

// Step 4: Fetch the contact name using the contact_id
let contact_name = match self
    .contact_repo
    .find_by_user_id_and_contact_id(
        user_id,
        Uuid::from_slice(
            updated_income
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

    // Step 5: Map the result to ResEntryIncomeDto
    let res_entry = ResEntryIncomeDto {
        id: String::from_utf8(updated_income.id).map_err(|e| {
            UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
        })?,
        transaction_type_name,
        amount: updated_income.amount,
        asset_name,
        contact_name,
        note: updated_income.note,
        created_at: updated_income
            .created_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
        updated_at: updated_income
            .updated_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
    };

    // Step 6: Return the response object
    Ok(res_entry)

    }

    async fn delete_income(&self, user_id: Uuid , transaction_id: Uuid) -> Result<(), UsecaseError>
    {   
        // Step 1: Check if the income record exists
    let income_exists = match self.income_repo.get_income_record_by_id(user_id, transaction_id).await {
        Ok(Some(_)) => true, // Income record exists
        Ok(None) => {
            return Err(UsecaseError::ResourceNotFound(format!(
                "Income record with ID '{}' not found",
                transaction_id
            )))
        } // Income record not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Delete the income record if it exists
    if income_exists {
        match self.income_repo.delete_income_record(user_id, transaction_id).await {
            Ok(_) => Ok(()), // Successfully deleted
            Err(err) => Err(UsecaseError::from(err)), // Handle repository errors
        }
    } else {
        Err(UsecaseError::ResourceNotFound(format!(
            "Income record with ID '{}' not found",
            transaction_id
        )))
    }
    }



    
    async fn get_all_income(&self, user_id: Uuid) -> Result<ResListIncomeDto, UsecaseError>
    {   
         // Step 1: Fetch all income records for the user from the repository
    let incomes = match self.income_repo.get_all_income_record_by_user(user_id).await {
        Ok(incomes) => incomes,
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Map the income records to ResEntryIncomeDto
    let mut data = Vec::new();
    for income in incomes {
        // Fetch the transaction type name using the transaction_type_id
        let transaction_type_name = match self
            .transaction_type_repo
            .get_transaction_type_by_id(
                user_id,
                Uuid::from_slice(&income.transaction_type_id).map_err(|e| {
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
                Uuid::from_slice(&income.asset_id).map_err(|e| {
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
                    income
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

        // Map the income record to ResEntryIncomeDto
        let res_entry = ResEntryIncomeDto {
            id: String::from_utf8(income.id).map_err(|e| {
                UsecaseError::from(RepositoryError::InvalidInput(e.to_string()))
            })?,
            transaction_type_name,
            amount: income.amount,
            asset_name,
            contact_name,
            note: income.note,
            created_at: income
                .created_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
            updated_at: income
                .updated_at
                .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
        };

        data.push(res_entry);
    }

    // Step 3: Create the response object
    let res_list = ResListIncomeDto {
        length: data.len() as i32,
        data,
    };

    // Step 4: Return the response object
    Ok(res_list)
    }

}

