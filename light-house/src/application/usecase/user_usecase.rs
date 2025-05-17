use std::sync::Arc;

use uuid::Uuid;

use crate::{application::usecase_req_impl::user_usecase::UserUsecase, domain::{dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ReqUpdateUserDto, ResMeDto, ResSignInDto}, req_repository::{auth_repository::AuthRepository, gender_repository::GenderRepository, user_repository::{UserRepositoryBase, UserRepositoryUtility}, user_role_repository::RoleManagementRepository}}, soc::soc_usecase::UsecaseError};





pub struct UserUseCase<T>
where
    T: UserRepositoryBase
        + UserRepositoryUtility
        + GenderRepository
        + RoleManagementRepository
        + AuthRepository
        + Send
        + Sync,
{
    user_repository: Arc<T>,
}

impl<T> UserUseCase<T>
where
    T: UserRepositoryBase
        + UserRepositoryUtility
        + GenderRepository
        + RoleManagementRepository
        + AuthRepository
        + Send
        + Sync,
{
    pub fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl<T> UserUsecase for UserUseCase<T>
where 
    T: UserRepositoryBase + UserRepositoryUtility + GenderRepository + RoleManagementRepository + AuthRepository + Send + Sync,
{
    async fn register_user(&self, user_dto: ReqSignUpDto) -> Result<ResMeDto, UsecaseError>
    {
        // Create a default response object
        let mut res_me = ResMeDto::default();

        // Step 1: Create the user in the database
        let created_user = match self.user_repository.create(user_dto).await {
            Ok(user) => user,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Fetch the gender name using the gender_id from the created user
        let gender_id = match Uuid::from_slice(&created_user.gender_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };

        let gender = match self.user_repository.get_gender_by_id(gender_id).await {
            Ok(gender) => gender,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 3: Fetch the role name using the role_id from the created user
        let role_id = match Uuid::from_slice(&created_user.user_role_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };

        let role = match self.user_repository.get_role_by_id(role_id).await {
            Ok(role) => role,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 4: Populate the response object
        res_me.id = match Uuid::from_slice(&created_user.id) {
            Ok(id) => id.to_string(),
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };
        res_me.email = created_user.email;
        res_me.first_name = created_user.first_name;
        res_me.username = created_user.username;
        res_me.last_name = created_user.last_name;
        res_me.gender = gender.map(|g| g.name).unwrap_or_default();
        res_me.user_role = role.map(|r| r.name).unwrap_or_default();

        // Step 5: Return the response object
        Ok(res_me)

    }

    async fn login(&self, user_dto: ReqSignInDto) -> Result<ResSignInDto, UsecaseError>
    {
        let result = self.user_repository.sign_in(user_dto).await;
        match result {
            Ok(res) => Ok(res.into_inner()),
            Err(err) => Err(UsecaseError::Unexpected(err)),
        }
    }

    async fn me(&self, user_id: Uuid) -> Result<ResMeDto, UsecaseError>
    {
        // Step 1: Fetch the user by ID
        let user = match self.user_repository.find_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(UsecaseError::ResourceNotFound(format!("User with ID '{}' not found", user_id))),
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Fetch the gender name using the gender_id from the user
        let gender_id = match Uuid::from_slice(&user.gender_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };

        let gender = match self.user_repository.get_gender_by_id(gender_id).await {
            Ok(Some(gender)) => gender.name,
            Ok(None) => String::from("Unknown"), // Default value if gender is not found
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 3: Fetch the role name using the user_role_id from the user
        let role_id = match Uuid::from_slice(&user.user_role_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };

        let user_role = match self.user_repository.get_role_by_id(role_id).await {
            Ok(Some(role)) => role.name,
            Ok(None) => String::from("Unknown"), // Default value if role is not found
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 4: Populate the response object
        let res_me = ResMeDto {
            id: match String::from_utf8(user.id) {
                Ok(id) => id,
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
            },
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            gender,
            user_role,
        };

        // Step 5: Return the response object
        Ok(res_me)
    }

    async fn update_user(&self, user_id: Uuid, user_dto: ReqUpdateUserDto) -> Result<ResMeDto, UsecaseError>
    {
        // Step 1: Update the user in the database
        let updated_user = match self.user_repository.update(user_dto, user_id).await {
            Ok(user) => user,
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 2: Fetch the gender name using the gender_id from the updated user
        let gender_id = match Uuid::from_slice(&updated_user.gender_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };

        let gender = match self.user_repository.get_gender_by_id(gender_id).await {
            Ok(Some(gender)) => gender.name,
            Ok(None) => String::from("Unknown"), // Default value if gender is not found
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 3: Fetch the role name using the user_role_id from the updated user
        let role_id = match Uuid::from_slice(&updated_user.user_role_id) {
            Ok(id) => id,
            Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
        };

        let user_role = match self.user_repository.get_role_by_id(role_id).await {
            Ok(Some(role)) => role.name,
            Ok(None) => String::from("Unknown"), // Default value if role is not found
            Err(err) => return Err(UsecaseError::from(err)),
        };

        // Step 4: Populate the response object
        let res_me = ResMeDto {
            id: match String::from_utf8(updated_user.id) {
                Ok(id) => id,
                Err(err) => return Err(UsecaseError::Unexpected(err.to_string())),
            },
            username: updated_user.username,
            email: updated_user.email,
            first_name: updated_user.first_name,
            last_name: updated_user.last_name,
            gender,
            user_role,
        };

        // Step 5: Return the response object
        Ok(res_me)
    }
}