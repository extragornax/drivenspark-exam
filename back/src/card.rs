use chrono::NaiveDate;
use serde::Serialize;

use crate::{
    database::DBAccessManager,
    domain::CardData,
    errors::{AppError, ErrorType},
};

pub fn respond<T: Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Ok(warp::reply::with_status(warp::reply::json(&err), status))
        }
    }
}

pub async fn create_card(
    _db_manager: DBAccessManager,
    _new_data: CardData,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match _db_manager.create_card(_new_data) {
        Ok(_branch) => respond(Ok(_branch), warp::http::StatusCode::CREATED),
        Err(_) => respond(
            Err(AppError::new("Internal server error", ErrorType::Internal)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn edit_card(
    _id: i64,
    _db_manager: DBAccessManager,
    _new_data: CardData,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match _db_manager.edit_card(_id, _new_data) {
        Ok(_branch) => respond(Ok(_branch), warp::http::StatusCode::OK),
        Err(_) => respond(
            Err(AppError::new("Internal server error", ErrorType::Internal)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn get_all_cards(
    _db_manager: DBAccessManager,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match _db_manager.get_all_cards() {
        Ok(_branch) => respond(Ok(_branch), warp::http::StatusCode::OK),
        Err(_) => respond(
            Err(AppError::new("Internal server error", ErrorType::Internal)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn get_card_by_id(
    _id: i64,
    _db_manager: DBAccessManager,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match _db_manager.get_card_by_id(_id) {
        Ok(_branch) => respond(Ok(_branch), warp::http::StatusCode::OK),
        Err(_) => respond(
            Err(AppError::new("Not found", ErrorType::NotFound)),
            warp::http::StatusCode::NOT_FOUND,
        ),
    }
}

#[derive(Serialize)]
struct ReplyTotal {
    total: i64,
    date: NaiveDate,
}

pub async fn check_total_duration_on_date(
    _date: NaiveDate,
    _db_manager: DBAccessManager,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match _db_manager.check_total_duration_on_date(_date) {
        Ok(_total) => respond(
            Ok(ReplyTotal {
                total: _total,
                date: _date,
            }),
            warp::http::StatusCode::OK,
        ),
        Err(_) => respond(
            Err(AppError::new("Not found", ErrorType::NotFound)),
            warp::http::StatusCode::NOT_FOUND,
        ),
    }
}

pub async fn delete_card(
    _id: i64,
    _db_manager: DBAccessManager,
) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    match _db_manager.delete_card(_id) {
        Ok(_branch) => respond(Ok(_branch), warp::http::StatusCode::ACCEPTED),
        Err(_) => respond(
            Err(AppError::new("Internal server error", ErrorType::Internal)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
