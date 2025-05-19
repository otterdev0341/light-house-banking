use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::contact_type_usecase::ContactTypeUseCase, usecase_req_impl::contact_type_usecase::ContactTypeUsecase}, domain::dto::contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto}, infrastructure::{database::mysql::impl_repository::contact_type_repo::ContactTypeRepositoryImpl, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn contact_type_routes() -> Vec<Route> {
    routes![
        create_contact_type,
        view_contact_type_by_id,
        view_all_contact_types,
        delete_contact_type_by_id,
        update_contact_type
    ]
}


#[utoipa::path(
    post,
    path = "/contact-type",
    summary = "Create a new contact type",
    description = "Create a new contact type",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateContactTypeDto,
    responses(
        (status = 201, description = "Contact type created successfully", body = ResEntryContactTypeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact Type"]
)]
#[post("/", data = "<dto>")]
pub async fn create_contact_type(
    user: AuthenticatedUser,
    dto: Json<ReqCreateContactTypeDto>,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ContactTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryContactTypeDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match contact_type_usecase.create_contact_type(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    get,
    path = "/contact-type/{contact_type_id}",
    summary = "Get a contact type by ID",
    description = "Get a contact type by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_type_id" = String, description = "The ID of the contact type"),
    ),
    responses(
        (status = 200, description = "Contact type found", body = ResEntryContactTypeDto),
        (status = 404, description = "Contact type not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact Type"]
)]
#[get("/<contact_type_id>")]
pub async fn view_contact_type_by_id(
    user: AuthenticatedUser,
    contact_type_id: Uuid,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ContactTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryContactTypeDto> {
    
    if contact_type_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }
    
    match contact_type_usecase.get_contact_type(user.id, contact_type_id).await {
        Ok(res) => {
            match res {
                Some(contact_type) => Ok(SuccessResponse(Status::Ok, contact_type)),
                None => Err(ErrorResponse(Status::NotFound, "Contact type not found".to_string())),
            } 
        },
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    get,
    path = "/contact-type",
    summary = "Get all contact types",
    description = "Get all contact types",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Contact types found", body = ResListContactTypeDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact Type"]
)]
#[get("/")]
pub async fn view_all_contact_types(
    user: AuthenticatedUser,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ContactTypeRepositoryImpl>>>,
) -> OtterResponse<ResListContactTypeDto> {
    match contact_type_usecase.get_all_contact_type(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}






#[utoipa::path(
    delete,
    path = "/contact-type/{contact_type_id}",
    summary = "Delete a contact type by ID",
    description = "Delete a contact type by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_type_id" = String, description = "The ID of the contact type"),
    ),
    responses(
        (status = 200, description = "Contact type deleted successfully", body = String),
        (status = 400, description = "Invalid contact type ID", body = ErrorResponse),
        (status = 404, description = "Contact type not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact Type"]
)]
#[delete("/<contact_type_id>")]
pub async fn delete_contact_type_by_id(
    user: AuthenticatedUser,
    contact_type_id: Uuid,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ContactTypeRepositoryImpl>>>,
) -> OtterResponse<String> {
    if contact_type_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }

    if contact_type_id == Uuid::nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }

    match contact_type_usecase.delete_contact_type(user.id, contact_type_id).await {
        Ok(_) => Ok(SuccessResponse(Status::Ok, format!("Contact type with ID {} deleted", contact_type_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}






#[utoipa::path(
    put,
    path = "/contact-type/{contact_type_id}",
    summary = "Update a contact type by ID",
    description = "Update a contact type by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_type_id" = String, description = "The ID of the contact type"),
    ),
    request_body = ReqUpdateContactTypeDto,
    responses(
        (status = 200, description = "Contact type updated successfully", body = ResEntryContactTypeDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 404, description = "Contact type not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact Type"]
)]
#[put("/<contact_type_id>", data = "<dto>")]
pub async fn update_contact_type(
    user: AuthenticatedUser,
    contact_type_id: Uuid,
    dto: Json<ReqUpdateContactTypeDto>,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ContactTypeRepositoryImpl>>>,
) -> OtterResponse<ResEntryContactTypeDto> {
    
    if contact_type_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }
    
    match contact_type_usecase.update_contact_type(user.id, contact_type_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}