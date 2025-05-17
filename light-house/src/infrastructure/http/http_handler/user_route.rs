use std::sync::Arc;

use rocket::{http::Status, post, put, routes, serde::json::Json, Route, State};
use validator::Validate;

use crate::{application::{usecase::{user_usecase::UserUseCase, wrapper::user_wrapper::UserRepositoryComposite}, usecase_req_impl::user_usecase::UserUsecase}, domain::dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ReqUpdateUserDto, ResMeDto, ResSignInDto}, infrastructure::http::{faring::{authentication::AuthenticatedUser, cors::options}, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}};




pub fn user_routes() -> Vec<Route> {
    routes![
        sign_in,
        sign_up,
        update_user,
        me,
        options
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


#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(
    req_sign_up: Json<ReqSignUpDto>,
    user_usecase: &State<Arc<UserUseCase<UserRepositoryComposite>>>
) -> OtterResponse<ResMeDto> {
    if let Err(errors) = req_sign_up.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }

    match user_usecase.register_user(req_sign_up.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }

}


#[put("/user", data = "<req_update_user>")]
pub async fn update_user(
    user: AuthenticatedUser,
    req_update_user: Json<ReqUpdateUserDto>,
    user_usecase: &State<Arc<UserUseCase<UserRepositoryComposite>>>
) -> OtterResponse<ResMeDto> {
    if let Err(errors) = req_update_user.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }

    match user_usecase.update_user(user.id, req_update_user.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}

#[post("/me")]
pub async fn me(user: AuthenticatedUser, user_usecase: &State<Arc<UserUseCase<UserRepositoryComposite>>>) -> OtterResponse<ResMeDto> {
    // This function is not implemented yet
    match user_usecase.me(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}