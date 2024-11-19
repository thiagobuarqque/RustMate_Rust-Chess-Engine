use std::sync::Mutex;

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    ai::ai_player::get_ai_move,
    game_bit_board::{
        _move::_move::Move, board::Board, move_generator::move_generator::MoveGenerator,
    },
};

#[get("/ai/move")]
pub async fn ai_move(
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>,
) -> impl Responder {
    let mut board = board.lock().unwrap();
    let mut move_generator = move_generator.lock().unwrap();

    if board.is_game_finished() {
        return HttpResponse::Ok().json(json!({
            "depth": 0,
            "duration": 0,
            "evaluation": 0,
            "aiMove": None::<Move>
        }));
    }

    let (duration, depth, evaluation, mut ai_move) =
        get_ai_move(&mut board, &mut move_generator, 3);

    let _ = board.move_piece(&mut ai_move);

    HttpResponse::Ok().json(json!({
        "depth": depth,
        "duration": duration,
        "evaluation": evaluation,
        "aiMove": ai_move
    }))
}
