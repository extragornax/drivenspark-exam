use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use thiserror::Error;
use warp::reject::Reject;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("jwt token not valid")]
    JWTToken,
    #[error("jwt token creation error")]
    JWTTokenCreation,
    #[error("no auth header")]
    NoAuthHeader,
    #[error("invalid auth header")]
    InvalidAuthHeader,
    #[error("no permission")]
    NoPermission,
    #[error("client not found")]
    ClientNotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

/// Handle route rejections
/// Based on status code
/// Swagger config:
///   - N/A (helper function)
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<AppError>() {
        match e.err_type {
            ErrorType::NotFound => (warp::http::StatusCode::NOT_FOUND, e.to_string()),
            ErrorType::Internal => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ErrorType::BadRequest => (warp::http::StatusCode::BAD_REQUEST, e.to_string()),
            ErrorType::DistantServer => {
                (warp::http::StatusCode::SERVICE_UNAVAILABLE, e.to_string())
            }
            ErrorType::CacheError => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ErrorType::Unauthorized => (warp::http::StatusCode::UNAUTHORIZED, e.to_string()),
            ErrorType::MissingRequiredField => (warp::http::StatusCode::BAD_REQUEST, e.to_string()),
            ErrorType::AlreadyExists => (warp::http::StatusCode::CONFLICT, e.to_string()),
        }
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::NoAuthHeader => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::NoPermission => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTToken => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Error::InvalidAuthHeader => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::ClientNotFound => (StatusCode::NOT_FOUND, e.to_string()),
            #[allow(unreachable_patterns)]
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::BAD_REQUEST, "Bad Request".to_string())
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        let message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("invalid type") {
                    format!("FIELD_ERROR: invalid type {}", cause)
                } else {
                    "Bad Request".to_string()
                }
            }
            None => "Bad Request".to_string(),
        };
        let code = StatusCode::BAD_REQUEST;

        (code, message)
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (
            StatusCode::PAYLOAD_TOO_LARGE,
            "Payload Too Large".to_string(),
        )
    } else if err.find::<warp::reject::InvalidQuery>().is_some() {
        (StatusCode::BAD_REQUEST, "Missing field".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        message,
        status: code.to_string(),
    });

    Ok(warp::reply::with_status(json, code))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorType {
    BadRequest,
    NotFound,
    Internal,
    DistantServer,
    CacheError,
    Unauthorized,
    MissingRequiredField,
    AlreadyExists,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl AppError {
    /// Create new AppError
    /// Swagger config:
    ///   - N/A (constructor)
    pub fn new(message: &str, err_type: ErrorType) -> AppError {
        AppError {
            message: message.to_string(),
            err_type,
        }
    }

    /// Convert diesel error to app error
    /// Swagger config:
    ///   - N/A (helper function)
    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> AppError {
        AppError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => ErrorType::BadRequest,
                diesel::result::Error::NotFound => ErrorType::NotFound,
                _ => ErrorType::Internal,
            },
        )
    }

    /// Convert AppError to HTTP status code
    /// Swagger config:
    ///   - N/A (helper function)
    pub fn to_status_code(&self) -> warp::http::StatusCode {
        match &self.err_type {
            ErrorType::BadRequest => warp::http::StatusCode::BAD_REQUEST,
            ErrorType::NotFound => warp::http::StatusCode::NOT_FOUND,
            ErrorType::DistantServer => warp::http::StatusCode::SERVICE_UNAVAILABLE,
            ErrorType::CacheError => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::Unauthorized => warp::http::StatusCode::UNAUTHORIZED,
            ErrorType::MissingRequiredField => warp::http::StatusCode::BAD_REQUEST,
            ErrorType::AlreadyExists => warp::http::StatusCode::CONFLICT,
            _ => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for AppError {}
