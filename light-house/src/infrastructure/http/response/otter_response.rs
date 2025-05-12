use rocket::{http::{ContentType, Status}, Request, Response};
use serde::Serialize;
use rocket::response::Responder;






// Example usage in a handler (assuming you have a User struct that derives Serialize)
/*
use rocket::serde::Serialize;
use rocket::http::Status;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[rocket::get("/users/<id>")]
async fn get_user(id: i32) -> OtterResponse<User> {
    // In a real app, you'd fetch the user from the database
    if id == 1 {
        let user = User { id: 1, name: "Alice".to_string() };
        success(Status::Ok, user) // Use the helper function
    } else {
        error(Status::NotFound, format!("User with id {} not found", id)) // Use the helper function
    }
}
*/




// Define a generic success response structure that wraps the actual data.
// This struct will be serialized to JSON.
#[derive(Serialize)]
struct SuccessPayload<T: Serialize> {
    data: T,
}



// Define a generic error response structure.
// This struct will be serialized to JSON.
#[derive(Serialize)]
struct ErrorPayload<E: Serialize> {
    error: E, // The 'error' field will now contain the serialized struct E
}


// The main SuccessResponse struct that holds the HTTP status and the data payload.
// It derives Responder to handle the HTTP response details.
#[derive(Debug)] // Add Debug for easier debugging
pub struct SuccessResponse<T: Serialize>(pub Status, pub T);

// Implement the Responder trait for SuccessResponse.
// This tells Rocket how to turn a SuccessResponse into an HTTP response.
impl<'r, T: Serialize> Responder<'r, 'static> for SuccessResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let SuccessResponse(status, data) = self;

        // Wrap the data in the standard success payload structure
        let payload = SuccessPayload { data };

        // Serialize the payload to a JSON string
        let json_body = serde_json::to_string(&payload).map_err(|e| {
            eprintln!("Failed to serialize success response: {}", e);
            Status::InternalServerError // Indicate a server error if serialization fails
        })?;

        // Build the HTTP response
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(json_body.len(), std::io::Cursor::new(json_body))
            .ok()
    }
}

// The main ErrorResponse struct that holds the HTTP status and the error message.
// It derives Responder.
#[derive(Debug)] // Add Debug for easier debugging
pub struct ErrorResponse<E: Serialize = String>(pub Status, pub E);


// Implement the Responder trait for ErrorResponse.
// This tells Rocket how to turn an ErrorResponse into an HTTP response.
impl<'r, E: Serialize> Responder<'r, 'static> for ErrorResponse<E> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let ErrorResponse(status, error_data) = self;

        // Wrap the error data in the standard error payload structure
        let payload = ErrorPayload { error: error_data };

        // Serialize the payload to a JSON string
        let json_body = serde_json::to_string(&payload).map_err(|e| {
            eprintln!("Failed to serialize error response: {}", e);
            Status::InternalServerError // Indicate a server error if serialization fails
        })?;

        // Build the HTTP response
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(json_body.len(), std::io::Cursor::new(json_body))
            .ok()
    }
}


// Type alias for the standard result type returned by handlers.
// This makes handler signatures cleaner.
pub type OtterResponse<T, E = String> = Result<SuccessResponse<T>, ErrorResponse<E>>;

// You might want to add some helper functions for convenience
pub fn success<T: Serialize>(status: Status, data: T) -> OtterResponse<T> {
    Ok(SuccessResponse(status, data))
}

// Helper for string-based errors (most common case)
pub fn error_message(status: Status, message: impl Into<String>) -> OtterResponse<()> {
    Err(ErrorResponse(status, message.into()))
}


// Helper for custom error struct based errors
pub fn error_struct<E: Serialize>(status: Status, error_data: E) -> OtterResponse<(), E> {
    Err(ErrorResponse(status, error_data))
}

