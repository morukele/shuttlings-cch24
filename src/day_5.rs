use actix_web::{post, HttpResponse};
use cargo_manifest::Manifest;
use std::str::FromStr;

#[post("/5/manifest")]
async fn manifest(data: String) -> HttpResponse {
    let content = Manifest::from_str(&data);
    let mut valid = 0;
    let mut order_list = String::new();

    match content {
        Ok(content) => {
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
        Err(e) => HttpResponse::BadRequest().body("Invalid manifest"),
    }
}
