#![recursion_limit = "256"]
extern crate openssl;
#[macro_use]
extern crate diesel;

use std::{env, net::IpAddr};
use warp::Filter;

use crate::{
    database::{get_pg_pool, PgPool},
    filters::api_filters,
};

mod card;
mod database;
mod domain;
mod errors;
mod filters;
mod schema;

pub struct ConfigMapReponse {
    pub ip: IpAddr,
    pub port: u16,
    pub database_url: String,
}

/// App config getter
pub fn get_app_config() -> ConfigMapReponse {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env not set");

    let app_ip: IpAddr = env::var("APP_IP")
        .expect("APP_IP env not set")
        .parse()
        .unwrap();

    let app_port = env::var("APP_PORT")
        .expect("APP_PORT env not set")
        .parse::<u16>()
        .unwrap();

    ConfigMapReponse {
        ip: app_ip,
        port: app_port,
        database_url: database_url,
    }
}

/// Main function, run and serves the routes
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    let config: ConfigMapReponse = get_app_config();
    let db_pool: PgPool = get_pg_pool(&config.database_url);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Origin",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Method",
            "Authorization",
            "Content-Type",
            "Origin",
            "Referer",
            "Sec-Fetch-Mode",
            "User-Agent",
        ])
        .allow_methods(vec!["OPTIONS", "POST", "GET", "PUT", "PATCH", "DELETE"]);

    // Get routes definition
    let routes = api_filters(db_pool)
        .recover(errors::handle_rejection)
        .with(cors);

    log::info!("Warp Server listening on {}...", config.port);
    warp::serve(routes).run((config.ip, config.port)).await
}
