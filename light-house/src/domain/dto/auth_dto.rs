use rocket::{serde::{Deserialize, Serialize}, Responder};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;



#[derive(Deserialize, Serialize, ToSchema, Validate, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignInDto{

    #[validate(email(message = "Invalid email"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String
}
#[derive(Serialize, Deserialize, Responder, ToSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ResSignInDto{
    
    pub token: String,
    
}


#[derive(Deserialize,Validate, ToSchema,Debug, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUpDto{
    #[validate(length(min = 6, message = "Username must be at least 6 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(length(min = 1, message = "First name must be at least 1 characters"))]
    pub first_name: String,


    #[validate(length(min = 1, message = "Last name must be at least 1 characters"))]
    pub last_name: String,

    #[validate(length(min = 3, message = "The Gender must not be empty"))]
    pub gender: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: u64,
}

#[derive(Deserialize,Validate, ToSchema,Debug)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateUserDto{
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,

}


#[derive(Deserialize, Serialize, Validate, ToSchema,Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct ResMeDto {
    pub id: String,
    pub gender: String,
    pub user_role: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String
}


#[derive(Deserialize, Serialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResMcpDto {
    pub mcp_token: String
}