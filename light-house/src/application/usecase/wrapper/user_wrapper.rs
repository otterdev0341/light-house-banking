use std::{ops::Deref, sync::Arc};

use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{domain::{dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ReqUpdateUserDto, ResSignInDto}, entities::{gender, user, user_role}, req_repository::{auth_repository::AuthRepository, gender_repository::GenderRepository, user_repository::{UserRepositoryBase, UserRepositoryUtility}, user_role_repository::RoleManagementRepository}}, infrastructure::database::mysql::impl_repository::{
    auth_repo::AuthRepositoryImpl, gender_repo::GenderRepositoryImpl,
    role_repo::RoleManagementRepositoryImpl, user_repo::UserRepositoryImpl,
}, soc::soc_repository::RepositoryError};

pub struct UserRepositoryComposite {
    pub user_repository: Arc<UserRepositoryImpl>,
    pub auth_repository: Arc<AuthRepositoryImpl>,
    pub role_repository: Arc<RoleManagementRepositoryImpl>,
    pub gender_repository: Arc<GenderRepositoryImpl>,
}

impl UserRepositoryComposite {
    pub fn user_repository(&self) -> &UserRepositoryImpl {
        &self.user_repository
    }

    pub fn auth_repository(&self) -> &AuthRepositoryImpl {
        &self.auth_repository
    }

    pub fn role_repository(&self) -> &RoleManagementRepositoryImpl {
        &self.role_repository
    }

    pub fn gender_repository(&self) -> &GenderRepositoryImpl {
        &self.gender_repository
    }
}

impl Deref for UserRepositoryComposite {
    type Target = UserRepositoryImpl;

    fn deref(&self) -> &Self::Target {
        &self.user_repository
    }
}

#[async_trait::async_trait]
impl UserRepositoryBase for UserRepositoryComposite {
    async fn create(&self, dto: ReqSignUpDto) -> Result<user::Model, RepositoryError>
    {
        self.user_repository.create(dto).await
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<user::Model>, RepositoryError>
    {
        self.user_repository.find_by_id(id).await
    }
    async fn find_all(&self) -> Result<Vec<user::Model>, RepositoryError>
    {
        self.user_repository.find_all().await
    }
    async fn update(&self, dto: ReqUpdateUserDto, user_id: Uuid) -> Result<user::Model, RepositoryError>
    {
        self.user_repository.update(dto, user_id).await
    }
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>
    {
        self.user_repository.delete(id).await
    }
}

#[async_trait::async_trait]
impl AuthRepository for UserRepositoryComposite {
    async fn sign_in(&self, sign_in_dto: ReqSignInDto) -> Result<Json<ResSignInDto>, String>{
        self.auth_repository.sign_in(sign_in_dto).await
    }
    
}


#[async_trait::async_trait]
impl RoleManagementRepository for UserRepositoryComposite {
    async fn has_role(&self, user_id: Uuid, role: &str) -> Result<bool, RepositoryError>
    {
        self.role_repository.has_role(user_id, role).await
    }
    async fn assign_role(&self, admin_id: Uuid, target_user_id: Uuid, role: &str) -> Result<(), RepositoryError>
    {
        self.role_repository.assign_role(admin_id, target_user_id, role).await
    }
    async fn revoke_role(&self, admin_id: Uuid, target_user_id: Uuid, role: &str) -> Result<(), RepositoryError>
    {
        self.role_repository.revoke_role(admin_id, target_user_id, role).await
    }
    async fn get_role_by_id(&self, role_id: Uuid) -> Result<Option<user_role::Model>, RepositoryError>
    {
        self.role_repository.get_role_by_id(role_id).await
    }

}


#[async_trait::async_trait]
impl GenderRepository for UserRepositoryComposite {
    async fn get_gender_by_id(&self, gender_id: Uuid) -> Result<Option<gender::Model>, RepositoryError>
    {   
        self.gender_repository.get_gender_by_id(gender_id).await
    }
    async fn get_all_gender(&self) -> Result<Vec<gender::Model>, RepositoryError>
    {
        self.gender_repository.get_all_gender().await
    }
}


#[async_trait::async_trait]
impl UserRepositoryUtility for UserRepositoryComposite{
    async fn find_by_username(&self, name: &str) -> Result<Option<user::Model>, RepositoryError>
    {
        self.user_repository.find_by_username(name).await
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, RepositoryError>
    {
        self.user_repository.find_by_email(email).await
    }

}