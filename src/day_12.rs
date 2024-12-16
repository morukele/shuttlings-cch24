use std::sync::Arc;

use crate::models::board::{Board, BoardValue};
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse,
};
use tokio::sync::RwLock;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(reset)
        .service(place)
        .service(random_board)
        .service(board);
}

#[get("/12/board")]
pub async fn board(data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    let data = data.read().await;

    HttpResponse::Ok().body(data.get_current_state().clone())
}

#[post("/12/reset")]
pub async fn reset(data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    let mut data = data.write().await;
    *data = Board::new();

    HttpResponse::Ok().body(data.get_current_state().clone())
}

#[post("/12/place/{team}/{column}")]
pub async fn place(
    info: web::Path<(BoardValue, u8)>,
    data: web::Data<Arc<RwLock<Board>>>,
) -> HttpResponse {
    let (team, column) = info.into_inner();
    println!("input: {:?}", (team, column));
    let mut data = data.write().await;

    if !(1..=4).contains(&column) {
        return HttpResponse::BadRequest().finish();
    }
    let column = (column - 1) as usize;

    if data.winner.is_some() {
        return HttpResponse::ServiceUnavailable().body(data.get_current_state().to_string());
    }

    let Some(y) = data
        .grid
        .iter()
        .rev()
        .position(|row| row[column] == BoardValue::Empty)
    else {
        return HttpResponse::ServiceUnavailable().body(data.get_current_state().to_string());
    };
    let y = data.grid.len() - y - 1;

    data.grid[y][column] = team;
    println!("{}", data.get_current_state());

    // horizontal
    if data.grid[y].iter().all(|&t| t == team) {
        data.winner = Some(team);
    }

    // vertical
    if (0..data.grid[0].len()).all(|y| data.grid[y][column] == team) {
        data.winner = Some(team);
    }

    // tl -> br
    if (0..data.grid.len()).all(|i| data.grid[i][i] == team) {
        data.winner = Some(team);
    }

    // br -> tl
    if (0..data.grid.len()).all(|i| data.grid[data.grid.len() - i - 1][i] == team) {
        data.winner = Some(team);
    }

    // no winner
    if data
        .grid
        .iter()
        .all(|r| r.iter().all(|&t| t != BoardValue::Empty))
    {
        data.winner = Some(BoardValue::Empty);
    }

    // Check winning one more time
    HttpResponse::Ok().body(data.get_current_state().to_string())
}

#[get("/12/random-board")]
pub async fn random_board(data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    let mut data = data.write().await;
    data.generate_random_board();

    HttpResponse::Ok().body(data.get_current_state().to_string())
}
