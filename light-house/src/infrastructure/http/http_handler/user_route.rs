use std::sync::Arc;

use rocket::{http::Status, post, route, routes, serde::json::Json, Route, State};
use validator::Validate;

use crate::{application::{usecase::{user_usecase::UserUseCase, wrapper::user_wrapper::UserRepositoryComposite}, usecase_req_impl::user_usecase::UserUsecase}, domain::{dto::auth_dto::{ReqSignInDto, ResSignInDto}, req_repository::user_repository::UserRepositoryUtility}, infrastructure::{database::mysql::impl_repository::{auth_repo::AuthRepositoryImpl, gender_repo::GenderRepositoryImpl, role_repo::RoleManagementRepositoryImpl, user_repo::UserRepositoryImpl}, http::response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}};




pub fn user_routes() -> Vec<Route> {
    routes![
        sign_in
        // get_users,
        // create_user,
        // update_user,
        // delete_user
    ]
}



#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    req_sign_in: Json<ReqSignInDto>,
    user_usecase: &State<Arc<UserUseCase<UserRepositoryComposite>>>
) -> OtterResponse<ResSignInDto>
{
     // field empty Bad request
    if let Err(errors) = req_sign_in.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match user_usecase.login(req_sign_in.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }

}