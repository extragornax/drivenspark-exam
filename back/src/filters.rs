use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use warp::Filter;

use crate::{
    database::{with_db_access_manager, PgPool},
    domain::CardData,
};

/// Filter for extracting the JSON body from the request.
/// Swagger config:
///   - N/A (helper function)
pub fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Route for creating a card.
/// Swagger config:
///   - Path: "/card"
///   - Method: POST
pub fn route_create_card(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card")
        .and(warp::post())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<CardData>())
        .and_then(super::card::create_card)
}

/// Route for editing a card.
/// Swagger config:
///   - Path: "/card/{id}"
///   - Method: PUT
pub fn route_edit_card(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<CardData>())
        .and_then(super::card::edit_card)
}

/// Route for deleting a card.
/// Swagger config:
///   - Path: "/card/{id}"
///   - Method: DELETE
pub fn route_delete_card(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(super::card::delete_card)
}

/// Route for getting a card by ID.
/// Swagger config:
///   - Path: "/card/{id}"
///   - Method: GET
pub fn route_get_card_by_id(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card" / i64)
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(super::card::get_card_by_id)
}

/// Route for checking total duration on a specific date.
/// Swagger config:
///   - Path: "/card/check/{date}"
///   - Method: GET
pub fn check_total_duration_on_date(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card" / "check" / NaiveDate)
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(super::card::check_total_duration_on_date)
}

/// Route for getting all cards.
/// Swagger config:
///   - Path: "/card"
///   - Method: GET
pub fn route_get_all_cards(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(super::card::get_all_cards)
}

/// Aggregates Warp Filters for the API routes.
/// Swagger config:
///   - Path: "/api"
///   - Sub-paths:
///     - "/card" (POST, PUT, DELETE, GET)
///     - "/card/{id}" (GET)
///     - "/card/check/{date}" (GET)
pub fn api_filters(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / ..).and(
        route_create_card(pool.clone())
            .or(route_edit_card(pool.clone()))
            .or(route_delete_card(pool.clone()))
            .or(route_get_all_cards(pool.clone()))
            .or(route_get_card_by_id(pool.clone()))
            .or(check_total_duration_on_date(pool.clone())),
    )
}
