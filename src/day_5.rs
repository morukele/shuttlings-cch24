use actix_web::{post, HttpRequest, HttpResponse};
use cargo_manifest::Manifest;

use std::str::FromStr;

#[post("/5/manifest")]
async fn manifest(req: HttpRequest, data: String) -> HttpResponse {
    let content_type = req.headers().get("content-type").unwrap().to_str().unwrap();
    let manifest_result: Result<Manifest, String> = match content_type {
        "application/toml" => {
            Manifest::from_str(&data).map_err(|_| "Invalid TOML manifest".to_string())
        }
        "application/yaml" => {
            serde_yml::from_str(&data).map_err(|_| "Invalid YAML manifest".to_string())
        }
        "application/json" => {
            serde_json::from_str(&data).map_err(|_| "Invalid JSON manifest".to_string())
        }
        _ => return HttpResponse::UnsupportedMediaType().finish(),
    };

    let content = match manifest_result {
        Ok(content) => content,
        Err(_) => return HttpResponse::BadRequest().body("Invalid manifest"),
    };

    let mut valid = 0;
    let mut order_list = String::new();

    let package = content.package.unwrap();
    let metadata = package.metadata;
    let keywords = package.keywords;
    let code = String::from("Christmas 2024");
    match keywords {
        Some(keywords) => {
            let k = keywords.as_local().unwrap().contains(&code);
            if !k {
                return HttpResponse::BadRequest().body("Magic keyword not provided");
            }
        }
        None => return HttpResponse::BadRequest().body("Magic keyword not provided"),
    }

    match metadata {
        Some(metadata) => {
            let metadata = metadata.as_table().unwrap();
            let orders = metadata.get("orders");

            match orders {
                Some(orders) => {
                    for order in orders.as_array() {
                        for o in order {
                            let ord = o.as_table().unwrap();
                            let item = ord.get("item").unwrap().as_str().unwrap();
                            let quantity = ord.get("quantity");
                            if let Some(q) = quantity {
                                let res = q.as_integer();
                                if let Some(r) = res {
                                    let output = format!("{}: {}\n", item, r);
                                    order_list.push_str(&output);
                                    valid += 1;
                                }
                            }
                        }
                    }
                    if valid > 0 {
                        // pop string
                        order_list.pop();
                        println!("{:?}", order_list);
                        HttpResponse::Ok().body(order_list)
                    } else {
                        HttpResponse::NoContent().finish()
                    }
                }
                None => HttpResponse::NoContent().finish(),
            }
        }
        None => HttpResponse::NoContent().finish(),
    }
}
