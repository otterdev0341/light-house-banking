use rocket::serde::json::Json;

use crate::domain::dto::auth_dto::{ReqSignInDto, ResSignInDto};





#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    
    async fn sign_in(&self, sign_in_dto: ReqSignInDto) -> Result<Json<ResSignInDto>, String>;

}