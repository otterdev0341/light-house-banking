use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use validator::Validate;

use crate::{application::{usecase::contact_usecase::ContactUseCase, usecase_req_impl::contact_usecase::ContactUsecase}, domain::dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto}, infrastructure::{database::mysql::impl_repository::contact_repo::ContactRepositoryImpl, http::{faring::authentication::AuthenticatedUser, response::otter_response::{ErrorResponse, OtterResponse, SuccessResponse}}}};






pub fn contact_routes() -> Vec<Route> {
    routes![
        create_contact,
        view_contact_by_id,
        view_all_contact,
        delete_contact_by_id,
        update_contact
    ]
}



#[utoipa::path(
    post,
    path = "/contact",
    summary = "Create a new contact",
    description = "Create a new contact",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateContactDto,
    responses(
        (status = 201, description = "Contact created successfully", body = ResEntryContactDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact"]
)]
#[post("/", data = "<dto>")]
async fn create_contact(
    user: AuthenticatedUser,
    dto: Json<ReqCreateContactDto>,
    contact_usecase: &State<Arc<ContactUseCase<ContactRepositoryImpl>>>,
) -> OtterResponse<ResEntryContactDto> {
    // field empty Bad request
    if let Err(errors) = dto.validate() {
        return Err(
            ErrorResponse(Status::BadRequest, format!("Validation errors: {:?}", errors))
        );
    }
    match contact_usecase.create_contact(user.id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Created, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}





#[utoipa::path(
    get,
    path = "/contact/{contact_id}",
    summary = "Get a contact by ID",
    description = "Get a contact by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_id" = String, description = "The ID of the contact to retrieve")
    ),
    responses(
        (status = 200, description = "Contact retrieved successfully", body = ResEntryContactDto),
        (status = 400, description = "Invalid contact ID", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Contact not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact"]
)]
#[get("/<contact_id>")]
pub async fn view_contact_by_id(
    user: AuthenticatedUser,
    contact_id: Uuid,
    contact_usecase: &State<Arc<ContactUseCase<ContactRepositoryImpl>>>,
) -> OtterResponse<ResEntryContactDto> {
    
    if contact_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }

    match contact_usecase.get_contact(user.id, contact_id).await {
        Ok(res) => {
            match res {
                Some(contact) => Ok(SuccessResponse(Status::Ok, contact)),
                None => Err(ErrorResponse(Status::NotFound, "Contact not found".to_string())),
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
    path = "/contact",
    summary = "Get all contacts",
    description = "Get all contacts",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Contacts retrieved successfully", body = ResListContactDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact"]
)]
#[get("/")]
pub async fn view_all_contact(
    user: AuthenticatedUser,
    contact_usecase: &State<Arc<ContactUseCase<ContactRepositoryImpl>>>,
) -> OtterResponse<ResListContactDto> {
    match contact_usecase.get_all_contact(user.id).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}




#[utoipa::path(
    delete,
    path = "/contact/{contact_id}",
    summary = "Delete a contact by ID",
    description = "Delete a contact by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_id" = String, description = "The ID of the contact to delete")
    ),
    responses(
        (status = 200, description = "Contact deleted successfully", body = String),
        (status = 400, description = "Invalid contact ID", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Contact not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact"]
)]
#[delete("/<contact_id>")]
pub async fn delete_contact_by_id(
    user: AuthenticatedUser,
    contact_id: Uuid,
    contact_usecase: &State<Arc<ContactUseCase<ContactRepositoryImpl>>>,
) -> OtterResponse<String> {

    if contact_id.is_nil() {
        return Err(ErrorResponse(Status::BadRequest, "Invalid contact type ID".to_string()));
    }

    match contact_usecase.delete_contact(user.id, contact_id).await {
        Ok(_) =>Ok(SuccessResponse(Status::Ok, format!("Contact with ID {} deleted successfully", contact_id))),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}





#[utoipa::path(
    put,
    path = "/contact/{contact_id}",
    summary = "Update a contact by ID",
    description = "Update a contact by ID",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_id" = String, description = "The ID of the contact to update")
    ),
    request_body = ReqUpdateContactDto,
    responses(
        (status = 200, description = "Contact updated successfully", body = ResEntryContactDto),
        (status = 400, description = "Validation errors", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Contact not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tags = ["Contact"]
)]
#[put("/<contact_id>", data = "<dto>")]
pub async fn update_contact(
    user: AuthenticatedUser,
    contact_id: Uuid,
    dto: Json<ReqUpdateContactDto>,
    contact_usecase: &State<Arc<ContactUseCase<ContactRepositoryImpl>>>,
) -> OtterResponse<ResEntryContactDto> {
    

    match contact_usecase.update_contact(user.id, contact_id, dto.into_inner()).await {
        Ok(res) => Ok(SuccessResponse(Status::Ok, res)),
        Err(err) => {
            let error_response = ErrorResponse(Status::InternalServerError, err.to_string());
            Err(error_response)
        }
    }
}