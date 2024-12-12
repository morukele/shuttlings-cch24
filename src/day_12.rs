use std::sync::Arc;

use crate::models::board::{Board, BoardValue};
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use tokio::sync::RwLock;

#[get("/12/board")]
pub async fn board(data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    let data = data.read().await;

    println!("{}", data.state);
    HttpResponse::Ok().body(data.state.clone())
}

#[post("/12/reset")]
pub async fn reset(data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    let mut data = data.write().await;
    *data = Board::new();

    println!("{}", data.state);

    HttpResponse::Ok().body(data.state.clone())
}

#[derive(Deserialize)]
struct Info {
    team: BoardValue,
    column: usize,
}

#[post("/12/place/{team}/{column}")]
pub async fn place(info: web::Path<Info>, data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    let info = info.into_inner();
    let mut data = data.write().await;

    // Check limits
    if info.team == BoardValue::Empty || info.column < 1 || info.column > 4 {
        return HttpResponse::BadRequest().finish();
    }

    // Check if column is full
    let column = &data.get_column(info.column);
    if !column.contains(&BoardValue::Empty) {
        // if the column does not contain empty, that means it is full
        return HttpResponse::ServiceUnavailable().finish();
    }

    // Check if game is over
    let game_status = &data.detect_winner();
    println!("Game Status: {:?}", game_status);
    match game_status {
        crate::models::board::GameStatus::GameOver(_) => {
            let result = data.state.clone();
            HttpResponse::ServiceUnavailable().body(result)
        }
        crate::models::board::GameStatus::InPlay => {
            data.place_item_in_column(info.team, info.column - 1);

            HttpResponse::Ok().body(data.get_current_state())
        }
    }
}
