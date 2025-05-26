use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::contact_usecase::ContactUsecase, domain::{dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto}, req_repository::contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}}, soc::soc_usecase::UsecaseError};






pub struct ContactUseCase<T>
where
    T: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync
{
    contact_repository: Arc<T>
}

impl<T> ContactUseCase<T>
where
    T: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync
{
    pub fn new(contact_repository: Arc<T>) -> Self {
        Self { contact_repository }
    }


}

#[async_trait::async_trait]
impl<T> ContactUsecase for ContactUseCase<T>
where
    T: ContactRepositoryBase + ContactRepositoryUtility + Send + Sync
{
    async fn create_contact(
        &self, 
        user_id: Uuid, 
        contact_dto: ReqCreateContactDto
    ) -> Result<ResEntryContactDto, UsecaseError> {
        log::debug!("Creating contact for user ID: {}", user_id);

        // Step 1: Validate and convert contact_type_id
        let contact_type_id = Uuid::parse_str(&contact_dto.contact_type_id)
            .map_err(|err| {
                log::error!("Invalid contact_type_id: {}", err);
                UsecaseError::InvalidData(format!("Invalid contact_type_id: {}", err))
            })?;

        // Step 2: Create the contact in the database
        let contact_created = match self
            .contact_repository
            .create(user_id, contact_dto)
            .await
        {
            Ok(contact) => contact,
            Err(err) => {
                log::error!("Failed to create contact: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 3: Fetch the contact type name using the contact_type_id
        let contact_type_name = match self.contact_repository.find_by_id(contact_type_id).await {
            Ok(Some(contact_type)) => contact_type.name,
            Ok(None) => String::from("Unknown"), // Default value if contact type is not found
            Err(err) => {
                log::error!("Failed to fetch contact type: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 4: Map the result to ResEntryContactDto
        let res_entry = ResEntryContactDto {
            id: match Uuid::from_slice(&contact_created.id) {
                Ok(id) => id.to_string(),
                Err(err) => {
                    log::error!("Invalid contact ID: {}", err);
                    return Err(UsecaseError::InvalidData(err.to_string()));
                }
            },
            name: contact_created.name,
            business_name: contact_created.business_name,
            phone: contact_created.phone,
            description: contact_created.description,
            contact_type_name,
            created_at: match contact_created.created_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
            updated_at: match contact_created.updated_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
        };

        log::debug!("Contact created successfully: {:?}", res_entry);

        // Step 5: Return the response object
        Ok(res_entry)
    }

    async fn get_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<Option<ResEntryContactDto>, UsecaseError> {
        // Step 1: Fetch the contact by user_id and contact_id from the repository
        let contact = match self.contact_repository.find_by_user_id_and_contact_id(user_id, contact_id).await {
            Ok(Some(contact)) => contact,
            Ok(None) => return Ok(None), // Contact not found
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Fetch the contact type name using the contact_type_id
        let contact_type_id = Uuid::from_slice(&contact.contact_type_id)
            .map_err(|err| {
                log::error!("Invalid contact_type_id: {}", err);
                UsecaseError::InvalidData(format!("Invalid contact_type_id: {}", err))
            })?;

        let contact_type_name = match self.contact_repository.find_by_id(contact_type_id).await {
            Ok(Some(contact_type)) => contact_type.name,
            Ok(None) => String::from("Unknown"), // Default value if contact type is not found
            Err(err) => {
                log::error!("Failed to fetch contact type: {}", err);
                return Err(UsecaseError::from(err));
            }
        };

        // Step 3: Map the contact details to ResEntryContactDto
        let res_entry = ResEntryContactDto {
            id: Uuid::from_slice(&contact.id)
                .map_err(|err| {
                    log::error!("Invalid contact ID: {}", err);
                    UsecaseError::InvalidData(err.to_string())
                })?
                .to_string(),
            name: contact.name,
            business_name: contact.business_name,
            phone: contact.phone,
            description: contact.description,
            contact_type_name,
            created_at: match contact.created_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
            updated_at: match contact.updated_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
        };

        // Step 4: Return the mapped contact details
        Ok(Some(res_entry))
    }

    async fn update_contact(
        &self, 
        user_id: Uuid,  
        contact_id: Uuid, 
        contact_dto: ReqUpdateContactDto
    ) -> Result<ResEntryContactDto, UsecaseError> {
        // Step 1: Call the repository to update the contact
        let updated_contact = self
            .contact_repository
            .update(contact_dto, user_id, contact_id)
            .await
            .map_err(|err| {
                log::error!("Failed to update contact: {}", err);
                UsecaseError::from(err)
            })?;

        // Step 2: Fetch the contact type name using the contact_type_id
        let contact_type_id = Uuid::from_slice(&updated_contact.contact_type_id)
            .map_err(|err| {
                log::error!("Invalid contact_type_id: {}", err);
                UsecaseError::InvalidData(format!("Invalid contact_type_id: {}", err))
            })?;
        
        

            let contact_type_name = match self.contact_repository.find_contact_type_by_id(user_id, contact_type_id).await {
                Ok(Some(contact_type)) => contact_type.name,
                Ok(None) => {
                    log::warn!(
                        "No contact type found for contact_type_id: {}",
                        contact_type_id
                    );
                    String::from("Unknown")
                },
                Err(err) => {
                    log::error!("Failed to fetch contact type: {}", err);
                    return Err(UsecaseError::from(err));
                }
            };

        // Step 3: Map the result to ResEntryContactDto
        let res_entry = ResEntryContactDto {
            id: Uuid::from_slice(&updated_contact.id)
                .map_err(|err| {
                    log::error!("Invalid contact ID: {}", err);
                    UsecaseError::InvalidData(err.to_string())
                })?
                .to_string(),
            name: updated_contact.name,
            business_name: updated_contact.business_name,
            phone: updated_contact.phone,
            description: updated_contact.description,
            contact_type_name,
            created_at: match updated_contact.created_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
            updated_at: match updated_contact.updated_at {
                Some(dt) => dt.to_string(),
                None => String::from(""),
            },
        };

        log::debug!("Contact updated successfully: {:?}", res_entry);

        // Step 4: Return the response object
        Ok(res_entry)
    }

    async fn delete_contact(
        &self, 
        user_id: Uuid , 
        contact_id: Uuid
    ) 
        -> Result<(), UsecaseError>
    {
        // Step 1: Check if the contact exists
        let contact_exists = match self.contact_repository.find_by_user_id_and_contact_id(user_id, contact_id).await {
            Ok(Some(_)) => true,
            Ok(None) => {
                return Err(UsecaseError::ResourceNotFound(format!(
                    "Contact with ID '{}' not found",
                    contact_id
                )))
            }
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Delete the contact if it exists
        if contact_exists {
            match self.contact_repository.delete(user_id, contact_id).await {
                Ok(_) => Ok(()),
                Err(err) => Err(UsecaseError::from(err)),
            }
        } else {
            Err(UsecaseError::ResourceNotFound(format!(
                "Contact with ID '{}' not found",
                contact_id
            )))
        }
    }

    async fn get_all_contact(
        &self, 
        user_id: Uuid
    ) -> Result<ResListContactDto, UsecaseError> {
        // Step 1: Fetch all contacts for the user from the repository
        let contacts = match self.contact_repository.find_all_by_user_id(user_id).await {
            Ok(contacts) => contacts,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Map the contacts to ResEntryContactDto
        let mut data = Vec::new();
        for contact in contacts {
            // Fetch the contact type name using the contact_type_id
            let contact_type_id = Uuid::from_slice(&contact.contact_type_id)
                .map_err(|err| {
                    log::error!("Invalid contact_type_id: {}", err);
                    UsecaseError::InvalidData(format!("Invalid contact_type_id: {}", err))
                })?;

            let contact_type_name = match self.contact_repository.find_contact_type_by_id(user_id,contact_type_id).await {
                Ok(Some(contact_type)) => contact_type.name,
                Ok(None) => String::from("Unknown"),
                Err(err) => {
                    log::error!("Failed to fetch contact type: {}", err);
                    return Err(UsecaseError::from(err));
                }
            };

            let res_entry = ResEntryContactDto {
                id: Uuid::from_slice(&contact.id)
                    .map_err(|err| {
                        log::error!("Invalid contact ID: {}", err);
                        UsecaseError::InvalidData(err.to_string())
                    })?
                    .to_string(),
                name: contact.name,
                business_name: contact.business_name,
                phone: contact.phone,
                description: contact.description,
                contact_type_name,
                created_at: match contact.created_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
                updated_at: match contact.updated_at {
                    Some(dt) => dt.to_string(),
                    None => String::from(""),
                },
            };

            data.push(res_entry);
        }

        // Step 3: Create the response object
        let res_list = ResListContactDto {
            length: data.len() as i32,
            data,
        };

        // Step 4: Return the response object
        Ok(res_list)
    }
}