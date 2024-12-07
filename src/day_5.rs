use actix_web::{http::header, post, HttpRequest, HttpResponse};
use cargo_manifest::Manifest;

#[derive(Default, serde::Deserialize)]
struct Metadata {
    #[serde(default)]
    orders: Vec<Order>,
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
struct Order {
    item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    quantity: Option<usize>,
}

#[post("/5/manifest")]
async fn manifest(req: HttpRequest, data: String) -> HttpResponse {
    // Extract and validate content type
    // Get package from manifest
    let Ok(Some(package)) = (match req
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
    {
        Some(ct) => match ct {
            "application/toml" => toml::from_str::<Manifest<Metadata>>(&data)
                .map_err(|_| "Invalid TOML Manifest".to_string()),
            "application/json" => serde_json::from_str::<Manifest<Metadata>>(&data)
                .map_err(|_| "Invalid JSON Manifest".to_string()),
            "application/yaml" => serde_yml::from_str::<Manifest<Metadata>>(&data)
                .map_err(|_| "Invalid YAML manifest".to_string()),
            _ => return HttpResponse::UnsupportedMediaType().finish(),
        },
        None => return HttpResponse::UnsupportedMediaType().finish(),
    })
    .map(|m| m.package) else {
        return HttpResponse::BadRequest().body("Invalid manifest");
    };

    // Check for code in keyword
    if !package
        .keywords
        .and_then(|k| k.as_local())
        .map(|k| k.contains(&"Christmas 2024".to_string()))
        .unwrap_or_default()
    {
        return HttpResponse::BadRequest().body("Magic keyword not provided");
    }

    let Some(orders) = package.metadata.map(|m| {
        m.orders
            .into_iter()
            .filter(|o| o.quantity.is_some())
            .map(|o| format!("{}: {}", o.item, o.quantity.unwrap()))
            .collect::<Vec<String>>()
    }) else {
        return HttpResponse::NoContent().finish();
    };

    // return no content if no valid order
    if orders.is_empty() {
        return HttpResponse::NoContent().finish();
    }

    // Final response
    HttpResponse::Ok().body(orders.join("\n"))
}
