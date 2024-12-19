use actix_web::{
    delete, get, post, put,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::uuid;
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, PartialEq, Eq)]
struct Quote {
    id: uuid::Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    author: String,
    quote: String,
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(reset)
        .service(cite)
        .service(remove)
        .service(undo)
        .service(draft);
}

#[post("/19/reset")]
pub async fn reset(db: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!("DELETE FROM quotes")
        .execute(db.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/19/cite/{id}")]
pub async fn cite(id: web::Path<uuid::Uuid>, db: Data<PgPool>) -> impl Responder {
    let uuid = id.into_inner();
    let result: Result<Quote, sqlx::Error> =
        sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE id = $1", uuid)
            .fetch_one(db.get_ref())
            .await;

    match result {
        Ok(quote) => HttpResponse::Ok().json(quote),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/19/remove/{id}")]
pub async fn remove(id: web::Path<uuid::Uuid>, db: Data<PgPool>) -> impl Responder {
    let uuid = id.into_inner();
    let result: Result<Quote, sqlx::Error> =
        sqlx::query_as!(Quote, "DELETE FROM quotes WHERE id = $1 RETURNING *", uuid)
            .fetch_one(db.get_ref())
            .await;

    match result {
        Ok(quote) => HttpResponse::Ok().json(quote),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[put("/19/undo/{id}")]
pub async fn undo(
    id: web::Path<uuid::Uuid>,
    payload: web::Json<Payload>,
    db: Data<PgPool>,
) -> impl Responder {
    let uuid = id.into_inner();
    let payload = payload.into_inner();
    let result: Result<Quote, sqlx::Error> = sqlx::query_as!(
        Quote, "UPDATE quotes SET author = $1, quote = $2, version = version + 1 WHERE id = $3 RETURNING *", &payload.author, &payload.quote, uuid
    )

    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(updated_quote) => HttpResponse::Ok().json(updated_quote),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/19/draft")]
pub async fn draft(body: web::Json<Payload>, db: Data<PgPool>) -> impl Responder {
    let uuid = uuid::Uuid::new_v4();
    let body = body.into_inner();
    let result: Result<Quote, sqlx::Error> = sqlx::query_as!(
        Quote,
        "INSERT INTO quotes (id, author, quote) VALUES ($1, $2, $3) RETURNING *",
        uuid,
        &body.author,
        &body.quote
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(quote) => HttpResponse::Created().json(quote),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
