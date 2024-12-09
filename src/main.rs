use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Redirect, ServiceConfig},
    Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttlings_cch24::{
    day_02::{dest, dest_v6, key, key_v6},
    day_05::manifest,
    day_09::{milk, new_rate_limiter, refill},
};
use std::sync::{Arc, Mutex};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
async fn seek() -> impl Responder {
    Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4").using_status_code(StatusCode::FOUND)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Setting up rate limiter for bucket
    let limiter = new_rate_limiter();
    let bucket = Arc::new(Mutex::new(limiter));

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(bucket.clone()))
            .service(hello_world)
            .service(seek)
            .service(dest)
            .service(key)
            .service(dest_v6)
            .service(key_v6)
            .service(milk)
            .service(refill)
            .service(manifest);
    };

    Ok(config.into())
}
