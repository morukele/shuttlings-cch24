use actix_web::{http::header::CONTENT_TYPE, post, web, HttpRequest, HttpResponse};
use leaky_bucket::RateLimiter;
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct ConversionUnits {
    #[serde(default)]
    liters: Option<f32>,
    #[serde(default)]
    gallons: Option<f32>,
    #[serde(default)]
    litres: Option<f32>,
    #[serde(default)]
    pints: Option<f32>,
}

#[post("/9/refill")]
async fn refill(limiter: web::Data<Arc<Mutex<RateLimiter>>>) -> HttpResponse {
    let mut bucket = limiter.lock().unwrap();
    *bucket = RateLimiter::builder()
        .max(5)
        .initial(5)
        .refill(1)
        .interval(Duration::from_secs(1))
        .build();

    HttpResponse::Ok().finish()
}

#[post("/9/milk")]
async fn milk(
    req: HttpRequest,
    limiter: web::Data<Arc<Mutex<RateLimiter>>>,
    data: String,
) -> HttpResponse {
    // get content header
    let header = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or_default();

    // acquire 1L of milk from tbe bucket
    let bucket = limiter.lock().unwrap();
    // println!("Bucket balance: {}", bucket.balance());

    match bucket.try_acquire(1) {
        true => match header {
            "application/json" => {
                let conversion_unit = serde_json::from_str::<ConversionUnits>(&data);
                println!("Payload {:?}", conversion_unit);
                match conversion_unit {
                    Ok(unit) => {
                        // process the request
                        match (unit.gallons, unit.liters, unit.litres, unit.pints) {
                            (Some(gallons), None, None, None) => {
                                let liters = gallons * 3.78541;
                                println!("litres: {liters}");
                                HttpResponse::Ok().json(json!({"liters": liters}))
                            }
                            (None, Some(liters), None, None) => {
                                // multiplication should expand the size of the float
                                let gallons = liters / 3.78541;
                                println!("gallons: {gallons}");
                                HttpResponse::Ok().json(json!({"gallons": gallons }))
                            }
                            (None, None, Some(litres), None) => {
                                // dealing with UK values
                                let pints = litres * 1.759754;
                                println!("pints: {}", pints);
                                HttpResponse::Ok().json(json!({"pints": pints}))
                            }
                            (None, None, None, Some(pints)) => {
                                // dealing with UK values
                                let litres = pints / 1.759754;
                                println!("litres: {}", litres);
                                HttpResponse::Ok().json(json!({"litres": litres}))
                            }
                            _ => HttpResponse::BadRequest().finish(), // handles all non interesting parts
                        }
                    }
                    Err(_) => {
                        // invalid json
                        HttpResponse::BadRequest().finish()
                    }
                }
            }
            _ => HttpResponse::Ok().body("Milk withdrawn\n"),
        },
        false => HttpResponse::TooManyRequests().body("No milk available\n"),
    }
}
