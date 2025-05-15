use uuid::Uuid;

use crate::{domain::dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ReqUpdateUserDto, ResMeDto, ResSignInDto}, soc::soc_usecase::UsecaseError};



#[async_trait::async_trait]
pub trait UserUsecase {
    async fn register_user(&self, user_dto: ReqSignUpDto) -> Result<ResMeDto, UsecaseError>;
    async fn login(&self, user_dto: ReqSignInDto) -> Result<ResSignInDto, UsecaseError>;
    async fn me(&self, user_id: Uuid) -> Result<ResMeDto, UsecaseError>;
    async fn update_user(&self, user_id: Uuid, user_dto: ReqUpdateUserDto) -> Result<ResMeDto, UsecaseError>;
}