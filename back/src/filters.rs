use serde::de::DeserializeOwned;
use warp::Filter;

use crate::{
    database::{with_db_access_manager, PgPool},
    domain::CardData,
};

pub fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn route_create_card(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card")
        .and(warp::post())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<CardData>())
        .and_then(super::card::create_card)
}

pub fn route_edit_card(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<CardData>())
        .and_then(super::card::edit_card)
}

pub fn route_delete_card(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("card" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(super::card::delete_card)
}

// Aggregates Warp Filters
pub fn api_filters(
    pool: PgPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / ..).and(
        route_create_card(pool.clone())
            .or(route_edit_card(pool.clone()))
            .or(route_delete_card(pool.clone())),
    )
}
