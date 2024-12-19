pub mod day_02;
pub mod day_05;
pub mod day_09;
pub mod day_12;
pub mod day_16;
pub mod day_19;
pub mod models;

use actix_web::{
    error, get,
    http::StatusCode,
    web::{self, Data, Redirect, ServiceConfig},
    HttpResponse, Responder,
};
use models::board::Board;
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
async fn seek() -> impl Responder {
    Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4").using_status_code(StatusCode::FOUND)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // migrating database
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    // Setting up rate limiter for bucket
    let limiter = day_09::new_rate_limiter();
    let bucket = Arc::new(Mutex::new(limiter));
    // Setting up board
    let grid = Arc::new(RwLock::new(Board::new()));

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(bucket.clone()))
            .app_data(Data::new(grid.clone()))
            .app_data(Data::new(pool))
            .service(hello_world)
            .service(seek)
            .configure(day_02::configure)
            .configure(day_05::configure)
            .configure(day_09::configure)
            .configure(day_12::configure)
            .configure(day_16::configure)
            .configure(day_19::configure)
            .app_data(web::PathConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
            }));
    };

    Ok(config.into())
}
