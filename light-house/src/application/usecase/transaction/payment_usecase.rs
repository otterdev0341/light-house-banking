use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::transaction_usecase::RecordPaymentUsecase, domain::{dto::transaction_dto::{ReqCreatePaymentDto, ReqUpdatePaymentDto, ResEntryPaymentDto, ResListPaymentDto}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}, expense_repository::{ExpenseRepositoryBase, ExpenseRepositoryUtill}, transaction_repository::{RecordPaymentRepositoryUtility, TransactionTypeRepositoryUtility}}}, soc::{soc_repository::RepositoryError, soc_usecase::UsecaseError}};





pub struct PaymentUseCase<T, A, C, TT, E>
where
    T: RecordPaymentRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
    E: ExpenseRepositoryBase + ExpenseRepositoryUtill + Send + Sync,
{
    payment_repo: Arc<T>,
    asset_repo: Arc<A>,
    contact_repo: Arc<C>,
    transaction_type_repo: Arc<TT>,
    expense_repo: Arc<E>,
}

impl<T, A, C, TT, E> PaymentUseCase<T, A, C, TT, E>
where
    T: RecordPaymentRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
    E: ExpenseRepositoryBase + ExpenseRepositoryUtill + Send + Sync,
{
    pub fn new(
        payment_repo: Arc<T>,
        asset_repo: Arc<A>,
        contact_repo: Arc<C>,
        transaction_type_repo: Arc<TT>,
        expense_repo: Arc<E>,
    ) -> Self {
        Self {
            payment_repo,
            asset_repo,
            contact_repo,
            transaction_type_repo,
            expense_repo
        }
    }
}


#[async_trait::async_trait]
impl<T, A, C, TT, E> RecordPaymentUsecase for PaymentUseCase<T, A, C, TT, E>
where
    T: RecordPaymentRepositoryUtility + Send + Sync,
    A: AssetRepositoryBase + AssetRepositoryUtility + Send + Sync,
    C: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync,
    TT: TransactionTypeRepositoryUtility + Send + Sync,
    E: ExpenseRepositoryBase + ExpenseRepositoryUtill + Send + Sync,
{

    async fn create_payment(&self, user_id: Uuid, payment_dto: ReqCreatePaymentDto) -> Result<ResEntryPaymentDto, UsecaseError>
    {   
        // Step 1: Create the payment record in the database
    let payment_created = match self.payment_repo.create_payment_record(user_id, payment_dto).await {
        Ok(payment) => payment,
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 2: Fetch the transaction type name using the transaction_type_id
    let transaction_type_name = match self
        .transaction_type_repo
        .get_transaction_type_by_id(
            user_id,
            Uuid::from_slice(&payment_created.transaction_type_id).map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(transaction_type)) => transaction_type.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 3: Fetch the expense name using the expense_id
    let expense_name = match self
        .expense_repo
        .find_by_user_id_and_expense_id(
            user_id,
            Uuid::from_slice(
                payment_created
                    .expense_id
                    .as_deref()
                    .ok_or_else(|| {
                        UsecaseError::from(RepositoryError::InvalidInput(
                            "expense_id is None".to_string(),
                        ))
                    })?,
            ).map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(expense)) => expense.description,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 4: Fetch the contact name using the contact_id
    let contact_name = match self
        .contact_repo
        .find_by_user_id_and_contact_id(
            user_id,
            Uuid::from_slice(
                payment_created
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
    
    let asset_id = match Uuid::from_slice(&payment_created.asset_id) {
        Ok(id) => id,
        Err(_) => return Err(UsecaseError::from(RepositoryError::InvalidInput("fail to convert uuid".to_string()))),
    };
    let asset_name = match self.asset_repo.find_by_id(user_id, asset_id).await {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };
    // Step 5: Map the result to ResEntryPaymentDto
    let res_entry = ResEntryPaymentDto {
        id: match Uuid::from_slice(&payment_created.id) {
            Ok(id) => id.to_string(),
            Err(_) => "Unknown".to_string(),
        },
        transaction_type_name,
        amount: payment_created.amount,
        expense_name,
        contact_name,
        asset_name,
        note: payment_created.note,
        // Add other fields as needed
        created_at: payment_created
            .created_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
        updated_at: payment_created
            .updated_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
    };

    // Step 6: Return the response object
    Ok(res_entry)
    }


    async fn get_payment(&self, user_id: Uuid , transaction_id: Uuid) -> Result<Option<ResEntryPaymentDto>, UsecaseError>
    {
        // Step 1: Fetch the payment record by user_id and transaction_id from the repository
    let payment = match self.payment_repo.get_payment_record_by_id(user_id, transaction_id).await {
        Ok(Some(payment)) => {
            // Step 2: Fetch the transaction type name using the transaction_type_id
            let transaction_type_name = match self
                .transaction_type_repo
                .get_transaction_type_by_id(
                    user_id,
                    Uuid::from_slice(&payment.transaction_type_id).map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(transaction_type)) => transaction_type.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 3: Fetch the expense name using the expense_id
            let expense_name = match self
                .asset_repo
                .find_by_id(
                    user_id,
                    Uuid::from_slice(
                        payment
                            .expense_id
                            .as_deref()
                            .ok_or_else(|| {
                                UsecaseError::from(RepositoryError::InvalidInput(
                                    "expense_id is None".to_string(),
                                ))
                            })?,
                    )
                    .map_err(|e| {
                        UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
                    })?,
                )
                .await
            {
                Ok(Some(expense)) => expense.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 4: Fetch the contact name using the contact_id
            let contact_name = match self
                .contact_repo
                .find_by_user_id_and_contact_id(
                    user_id,
                    Uuid::from_slice(
                        payment
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

            let asset_id = match Uuid::from_slice(&payment.asset_id) {
                Ok(id) => id,
                Err(_) => return Err(UsecaseError::from(RepositoryError::InvalidInput("fail to convert uuid".to_string()))),
            };
            let asset_name = match self.asset_repo.find_by_id(user_id, asset_id).await {
                Ok(Some(asset)) => asset.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => return Err(UsecaseError::from(err)),
            };

            // Step 5: Map the result to ResEntryPaymentDto
            Some(ResEntryPaymentDto {
                id: match Uuid::from_slice(&payment.id) {
                    Ok(id) => id.to_string(),
                    Err(_) => "Unknown".to_string(),
                },
                transaction_type_name,
                amount: payment.amount,
                expense_name,
                contact_name,
                asset_name,
                note: payment.note,
                created_at: payment
                    .created_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
                updated_at: payment
                    .updated_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
            })
        }
        Ok(None) => return Ok(None), // Payment record not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 6: Return the mapped payment details
    Ok(payment)
    }

    async fn update_payment(&self, user_id: Uuid,  transaction_id: Uuid, payment_dto: ReqUpdatePaymentDto) -> Result<ResEntryPaymentDto, UsecaseError>
    {
        // Step 1: Call the repository to update the payment record
    let updated_payment = match self
    .payment_repo
    .update_payment_record(user_id, transaction_id, payment_dto)
    .await
    {
        Ok(payment) => payment,
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 2: Fetch the transaction type name using the transaction_type_id
    let transaction_type_name = match self
        .transaction_type_repo
        .get_transaction_type_by_id(
            user_id,
            Uuid::from_slice(&updated_payment.transaction_type_id).map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(transaction_type)) => transaction_type.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 3: Fetch the expense name using the expense_id
    let expense_name = match self
        .asset_repo
        .find_by_id(
            user_id,
            Uuid::from_slice(
                updated_payment
                    .expense_id
                    .as_deref()
                    .ok_or_else(|| {
                        UsecaseError::from(RepositoryError::InvalidInput(
                            "expense_id is None".to_string(),
                        ))
                    })?,
            )
            .map_err(|e| {
                UsecaseError::from(RepositoryError::from(sea_orm::DbErr::Custom(e.to_string())))
            })?,
        )
        .await
    {
        Ok(Some(expense)) => expense.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 4: Fetch the contact name using the contact_id
    let contact_name = match self
        .contact_repo
        .find_by_user_id_and_contact_id(
            user_id,
            Uuid::from_slice(
                updated_payment
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


    let asset_id = match Uuid::from_slice(&updated_payment.asset_id) {
        Ok(id) => id,
        Err(_) => return Err(UsecaseError::from(RepositoryError::InvalidInput("fail to convert uuid".to_string()))),
    };
    let asset_name = match self.asset_repo.find_by_id(user_id, asset_id).await {
        Ok(Some(asset)) => asset.name,
        Ok(None) => String::from("Unknown"),
        Err(err) => return Err(UsecaseError::from(err)),
    };

    // Step 5: Map the result to ResEntryPaymentDto
    let res_entry = ResEntryPaymentDto {
        id: match Uuid::from_slice(&updated_payment.id) {
            Ok(id) => id.to_string(),
            Err(_) => "Unknown".to_string(),
        },
        transaction_type_name,
        amount: updated_payment.amount,
        expense_name,
        contact_name,
        asset_name,
        note: updated_payment.note,
        created_at: updated_payment
            .created_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
        updated_at: updated_payment
            .updated_at
            .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
    };

    // Step 6: Return the response object
    Ok(res_entry)

    }

    async fn delete_payment(&self, user_id: Uuid , transaction_id: Uuid) -> Result<(), UsecaseError>
    {
        // Step 1: Check if the payment record exists
    let payment_exists = match self.payment_repo.get_payment_record_by_id(user_id, transaction_id).await {
        Ok(Some(_)) => true, // Payment record exists
        Ok(None) => {
            return Err(UsecaseError::ResourceNotFound(format!(
                "Payment record with ID '{}' not found",
                transaction_id
            )))
        } // Payment record not found
        Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
    };

    // Step 2: Delete the payment record if it exists
    if payment_exists {
        match self.payment_repo.delete_payment_record(user_id, transaction_id).await {
            Ok(_) => Ok(()), // Successfully deleted
            Err(err) => Err(UsecaseError::from(err)), // Handle repository errors
        }
    } else {
        Err(UsecaseError::ResourceNotFound(format!(
            "Payment record with ID '{}' not found",
            transaction_id
        )))
    }
    }

    async fn get_all_payment(&self, user_id: Uuid) -> Result<ResListPaymentDto, UsecaseError> {
        log::info!("Getting all payments for user ID: {}", user_id);

        // Step 1: Fetch all payment records for the user from the repository
        let payments = match self.payment_repo.get_all_payment_record_by_user(user_id).await {
            Ok(payments) => payments,
            Err(err) => {
                log::error!("Failed to fetch payment records: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 2: Map the payment records to ResEntryPaymentDto
        let mut data = Vec::new();
        for payment in payments {
            log::debug!("Processing payment record: {:?}", payment);

            // Fetch the transaction type name using the transaction_type_id
            let transaction_type_name = match self
                .transaction_type_repo
                .get_transaction_type_by_id(
                    user_id,
                    Uuid::from_slice(&payment.transaction_type_id).map_err(|e| {
                        log::error!("Failed to parse transaction_type_id: {}", e);
                        UsecaseError::from(RepositoryError::InvalidInput("Invalid transaction_type_id".to_string()))
                    })?,
                )
                .await
            {
                Ok(Some(transaction_type)) => transaction_type.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => {
                    log::error!("Failed to fetch transaction type: {}", err);
                    return Err(UsecaseError::from(err));
                }
            };

            // Fetch the expense name using the expense_id
            let expense_name = match payment.expense_id.as_deref() {
                Some(expense_id) => {
                    let expense_uuid = Uuid::from_slice(expense_id).map_err(|e| {
                        log::error!("Invalid expense_id: {}", e);
                        UsecaseError::from(RepositoryError::InvalidInput("Invalid expense_id".to_string()))
                    })?;
                    match self.expense_repo.find_by_user_id_and_expense_id(user_id, expense_uuid).await {
                        Ok(Some(expense)) => expense.description,
                        Ok(None) => String::from("Unknown"),
                        Err(err) => {
                            log::error!("Failed to fetch expense: {}", err);
                            return Err(UsecaseError::from(err));
                        }
                    }
                }
                None => String::from("Unknown"),
            };

            // Fetch the contact name using the contact_id
            let contact_name = match payment.contact_id.as_deref() {
                Some(contact_id) => {
                    let contact_uuid = Uuid::from_slice(contact_id).map_err(|e| {
                        log::error!("Invalid contact_id: {}", e);
                        UsecaseError::from(RepositoryError::InvalidInput("Invalid contact_id".to_string()))
                    })?;
                    match self.contact_repo.find_by_user_id_and_contact_id(user_id, contact_uuid).await {
                        Ok(Some(contact)) => contact.name,
                        Ok(None) => String::from("Unknown"),
                        Err(err) => {
                            log::error!("Failed to fetch contact: {}", err);
                            return Err(UsecaseError::from(err));
                        }
                    }
                }
                None => String::from("Unknown"),
            };

            let asset_id = match Uuid::from_slice(&payment.asset_id) {
                Ok(id) => id,
                Err(e) => {
                    log::error!("Invalid asset_id: {}", e);
                    return Err(UsecaseError::from(RepositoryError::InvalidInput("Invalid asset_id".to_string())));
                }
            };
            let asset_name = match self.asset_repo.find_by_id(user_id, asset_id).await {
                Ok(Some(asset)) => asset.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => {
                    log::error!("Failed to fetch asset: {}", err);
                    return Err(UsecaseError::from(err));
                }
            };

            // Map the payment record to ResEntryPaymentDto
            let res_entry = ResEntryPaymentDto {
                id: match Uuid::from_slice(&payment.id) {
                    Ok(id) => id.to_string(),
                    Err(_) => "Unknown".to_string(),
                },
                transaction_type_name,
                amount: payment.amount,
                expense_name,
                contact_name,
                asset_name,
                note: payment.note,
                created_at: payment
                    .created_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_string()),
                updated_at: payment
                    .updated_at
                    .map_or_else(|| "Unknown".to_string(), |dt| dt.to_rfc3339()),
            };

            data.push(res_entry);
        }

        // Step 3: Create the response object
        let res_list = ResListPaymentDto {
            length: data.len() as i32,
            data,
        };

        

        // Step 4: Return the response object
        Ok(res_list)
    }
}