use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::contact_type_usecase::ContactTypeUsecase, domain::{dto::contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto}, req_repository::contact_type_repository::{ContactTypeRepositoryBase, ContactTypeRepositoryUtility}}, soc::soc_usecase::UsecaseError};




pub struct ContactTypeUseCase<T>
where
    T : ContactTypeRepositoryBase + ContactTypeRepositoryUtility + Send + Sync,
{
    contact_type_repo: Arc<T>,
}

impl<T> ContactTypeUseCase<T>
where
    T: ContactTypeRepositoryBase + ContactTypeRepositoryUtility + Send + Sync,
{
    pub fn new(contact_type_repo: Arc<T>) -> Self {
        Self { contact_type_repo }
    }
}


#[async_trait::async_trait]
impl<T> ContactTypeUsecase for ContactTypeUseCase<T>
where
    T: ContactTypeRepositoryBase + ContactTypeRepositoryUtility + Send + Sync,
{
    async fn create_contact_type(
        &self, 
        user_id: Uuid, 
        contact_type_dto: ReqCreateContactTypeDto
    ) -> Result<ResEntryContactTypeDto, UsecaseError> {
        // Step 1: Create the contact type in the database
        let contact_type_created = match self.contact_type_repo.create(user_id, contact_type_dto).await {
            Ok(contact_type) => contact_type,
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Map the result to ResEntryContactTypeDto
        let res_entry = ResEntryContactTypeDto {
            id: match Uuid::from_slice(&contact_type_created.id) {
                Ok(id) => id.to_string(), // Convert the ID from Vec<u8> to String
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UUID error
            },
            name: contact_type_created.name, // Contact type name
            created_at: match contact_type_created.created_at {
                Some(dt) => dt.to_string(), // Convert created_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
            updated_at: match contact_type_created.updated_at {
                Some(dt) => dt.to_string(), // Convert updated_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
        };

        // Step 3: Return the response object
        Ok(res_entry)
    }

    async fn get_contact_type(
        &self, 
        user_id: Uuid, 
        contact_type_id: Uuid
    ) -> Result<Option<ResEntryContactTypeDto>, UsecaseError> {
        // Step 1: Fetch the contact type by user_id and contact_type_id from the repository
        let contact_type = match self.contact_type_repo.find_by_id(user_id, contact_type_id).await {
            Ok(Some(contact_type)) => contact_type,
            Ok(None) => return Ok(None), // Contact type not found
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Map the contact type details to ResEntryContactTypeDto
        let res_entry = ResEntryContactTypeDto {
            id: match Uuid::from_slice(&contact_type.id) {
                Ok(id) => id.to_string(), // Convert the ID from Vec<u8> to String
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())), // Handle invalid UUID error
            },
            name: contact_type.name, // Contact type name
            created_at: match contact_type.created_at {
                Some(dt) => dt.to_string(), // Convert created_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
            updated_at: match contact_type.updated_at {
                Some(dt) => dt.to_string(), // Convert updated_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
        };

        // Step 3: Return the mapped contact type details
        Ok(Some(res_entry))
    }


    async fn update_contact_type(
        &self, 
        user_id: Uuid,  
        contact_type_id: Uuid, 
        contact_type_dto: ReqUpdateContactTypeDto
    ) -> Result<ResEntryContactTypeDto, UsecaseError> {
        // Step 1: Call the repository to update the contact type
        let updated_contact_type = match self
            .contact_type_repo
            .update(contact_type_dto, user_id, contact_type_id)
            .await
        {
            Ok(contact_type) => contact_type,
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Map the updated contact type to ResEntryContactTypeDto
        let res_entry = ResEntryContactTypeDto {
            id: match Uuid::from_slice(&updated_contact_type.id) {
                Ok(id) => id.to_string(), // Convert the ID from Vec<u8> to String
                Err(err) => return Err(UsecaseError::InvalidData(err.to_string())), // Handle invalid UUID error
            },
            name: updated_contact_type.name, // Contact type name
            created_at: match updated_contact_type.created_at {
                Some(dt) => dt.to_string(), // Convert created_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
            updated_at: match updated_contact_type.updated_at {
                Some(dt) => dt.to_string(), // Convert updated_at to String if present
                None => String::from(""),   // Default to an empty string if None
            },
        };

        // Step 3: Return the response object
        Ok(res_entry)
    }

    async fn delete_contact_type(
        &self, 
        user_id: Uuid , 
        contact_type_id: Uuid
    ) 
        -> Result<(), UsecaseError>
    {
         // Step 1: Check if the contact type exists
         let contact_type_exists = match self.contact_type_repo.find_by_id(user_id, contact_type_id).await {
            Ok(Some(_)) => true, // Contact type exists
            Ok(None) => {
                return Err(UsecaseError::ResourceNotFound(format!(
                    "Contact type with ID '{}' not found",
                    contact_type_id
                )))
            } // Contact type not found
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Delete the contact type if it exists
        if contact_type_exists {
            match self.contact_type_repo.delete(user_id, contact_type_id).await {
                Ok(_) => Ok(()), // Successfully deleted
                Err(err) => Err(UsecaseError::from(err)), // Handle repository errors
            }
        } else {
            Err(UsecaseError::ResourceNotFound(format!(
                "Contact type with ID '{}' not found",
                contact_type_id
            )))
        }
    }

    async fn get_all_contact_type(
        &self, 
        user_id: Uuid
    ) -> Result<ResListContactTypeDto, UsecaseError> {
        // Step 1: Fetch all contact types for the user from the repository
        let contact_types = match self.contact_type_repo.find_all_by_user_id(user_id).await {
            Ok(contact_types) => contact_types,
            Err(err) => return Err(UsecaseError::from(err)), // Handle repository errors
        };

        // Step 2: Map the contact types to ResEntryContactTypeDto
        let mut data = Vec::new();
        for contact_type in contact_types {
            let id = match Uuid::from_slice(&contact_type.id) {
                Ok(id) => id.to_string(), // Convert the ID from Vec<u8> to String
                Err(err) => return Err(UsecaseError::InvalidData(err.to_string())), // Handle invalid UUID error
            };

            let created_at = match contact_type.created_at {
                Some(dt) => dt.to_string(), // Convert created_at to String if present
                None => String::from(""),   // Default to an empty string if None
            };

            let updated_at = match contact_type.updated_at {
                Some(dt) => dt.to_string(), // Convert updated_at to String if present
                None => String::from(""),   // Default to an empty string if None
            };

            let res_entry = ResEntryContactTypeDto {
                id,
                name: contact_type.name,
                created_at,
                updated_at,
            };

            data.push(res_entry);
        }

        // Step 3: Create the response object
        let res_list = ResListContactTypeDto {
            length: data.len() as i32,
            data,
        };

        // Step 4: Return the response object
        Ok(res_list)
    }
}
