use crate::{domain::dto::auth_dto::{ReqSignInDto, ResSignInDto}, soc::soc_repository::RepositoryError};





#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    
    async fn sign_in(&self, sign_in_dto: ReqSignInDto) -> Result<ResSignInDto, RepositoryError>;

}