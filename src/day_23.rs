use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse,
};
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};

const VALID_COLORS: [&str; 3] = ["red", "blue", "purple"];

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(star)
        .service(preset)
        .service(ornament)
        .service(lockfile);
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[get("/23/star")]
pub async fn star() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"<div id="star" class="lit"></div>"#)
}

#[get("/23/present/{color}")]
pub async fn preset(color: web::Path<String>) -> HttpResponse {
    let mut color = color.into_inner();
    color = html_escape(&color);

    if !VALID_COLORS.contains(&color.as_str()) {
        return HttpResponse::ImATeapot().finish();
    }

    // determine the next color in the cycle
    let next_color = {
        let current_index = VALID_COLORS.iter().position(|&c| c == color).unwrap();
        VALID_COLORS[(current_index + 1) % VALID_COLORS.len()]
    };

    // Generate the HTML with the new color and hx-get attribute
    let html = format!(
        r#"
            <div class="present {color}" hx-get="/23/present/{next_color}" hx-swap="outerHTML">
                <div class="ribbon"></div>
                <div class="ribbon"></div>
                <div class="ribbon"></div>
                <div class="ribbon"></div>
            </div>
        "#,
        color = color,
        next_color = next_color
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[get("/23/ornament/{state}/{n}")]
pub async fn ornament(info: web::Path<(String, String)>) -> HttpResponse {
    let (mut state, mut n) = info.into_inner();
    state = html_escape(&state);
    n = html_escape(&n);

    // checking for valid state
    if state != "on" && state != "off" {
        return HttpResponse::ImATeapot().finish();
    }

    // Determining the next state, and class
    let next_state = if state == "on" { "off" } else { "on" };
    let class = if state == "on" {
        "ornament on"
    } else {
        "ornament"
    };

    // Generate the HTML
    let html = format!(
        r#"
            <div class="{class}" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{next_state}/{n}" hx-swap="outerHTML"></div>
        "#,
        n = n,
        class = class,
        next_state = next_state
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    checksum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockFile {
    package: Vec<Package>,
}

#[post("/23/lockfile")]
pub async fn lockfile(mut payload: Multipart) -> HttpResponse {
    // Extract data from the payload
    let mut data = String::new();
    while let Some(field) = payload.next().await {
        let mut field = match field {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().finish(),
        };

        // checking name
        if field.name().unwrap() == "lockfile" {
            // Add file to the container
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(_) => return HttpResponse::BadRequest().finish(),
                };

                data.push_str(&String::from_utf8_lossy(&chunk));
            }
        }
    }

    // Convert file to valid cargo manifest
    let lockfile = match toml::from_str::<LockFile>(&data) {
        Ok(lockfile) => lockfile,
        Err(e) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let mut html = String::new();

    for package in lockfile.package {
        if let Some(checksum) = package.checksum {
            match generate_div(&checksum) {
                Ok(div) => {
                    html.push_str(&div);
                }
                Err(_) => return HttpResponse::UnprocessableEntity().finish(),
            }
        } else {
            // skip the current iteration
            continue;
        }
    }

    HttpResponse::Ok().content_type("text/html").body(html)
}

fn generate_div(checksum: &str) -> Result<String, String> {
    // Check cases
    if checksum.len() < 10
        || checksum.chars().any(|c| {
            let c = c.to_ascii_lowercase();

            !('0'..='f').contains(&c) || (c > '9' && c < 'a')
        })
    {
        return Err(format!("Invalid Checksum: {}", checksum));
    }

    let color = &checksum[0..6];
    u32::from_str_radix(color, 16).map_err(|e| e.to_string())?;
    let top = u8::from_str_radix(&checksum[6..8], 16).map_err(|e| e.to_string())?;
    let left = u8::from_str_radix(&checksum[8..10], 16).map_err(|e| e.to_string())?;

    Ok(format!(
        r#"<div style="background-color:#{color};top:{top}px;left:{left}px;"></div>"#
    ))
}
