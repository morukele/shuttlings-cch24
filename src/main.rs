use actix_web::{
    get,
    http::StatusCode,
    web::{service, Redirect, ServiceConfig},
    Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttlings_cch24::{
    day_2::{dest, dest_v6, key, key_v6},
    day_5::manifest,
};

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
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(seek)
            .service(dest)
            .service(key)
            .service(dest_v6)
            .service(key_v6)
            .service(manifest);
    };

    Ok(config.into())
}
