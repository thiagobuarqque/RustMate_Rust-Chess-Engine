use std::{sync::{Mutex, MutexGuard}, time::Instant};

use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::{json, Value};

use crate::{
    dto::dtos::{FenDTO, MoveDTO, MovesCountDTO}, game_bit_board::{_move::{_move::Move, moves_counter::count_moves}, board::Board, enums::Color, move_generator::move_generator::MoveGenerator, utils::utils::get_piece_letter}
};

#[get("/board")]
pub async fn get_board(board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>) -> impl Responder {
    get_board_response(&mut board.lock().unwrap(), &mut move_generator.lock().unwrap())
}

#[post("/board/moves/count")]
pub async fn get_move_generation_count(
    piece_move: web::Json<MovesCountDTO>,
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>
) -> impl Responder {
    let mut board = &mut board.lock().unwrap();
    let mut move_generator = &mut move_generator.lock().unwrap();

    let start = Instant::now();

    let nodes_searched = count_moves(&mut board, piece_move.depth, false, &mut move_generator);

    HttpResponse::Ok().json(json!({
        "moves": nodes_searched,
        "elapsedTime": start.elapsed().as_millis(),
    }))
}

#[post("/board/move/piece")]
pub async fn move_piece(
    _move1: web::Json<MoveDTO>,
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>
) -> impl Responder {
    let board = &mut board.lock().unwrap();

    let mut move_generator= move_generator.lock().unwrap();

    let moves = move_generator
        .get_moves(board).unwrap();

    let moves = moves.iter().filter(|_move2| _move2.get_from() == _move1.from && _move2.get_to() == _move1.to).collect::<Vec<&Move>>();

    if moves.is_empty() {
        return HttpResponse::NotFound().json(json!({
            "message": "Move not found"
        }));
    }

    let mut _move = (*moves.first().unwrap()).clone();

    if moves.len() > 1 {
        for _move2 in moves {
            if _move2.get_flags() == _move1.flags {
                _move = _move2.clone();
                break;
            }
        }
    }

    board.move_piece(&mut _move);
    
    get_board_response(board, &mut move_generator)

    // HttpResponse::NotFound().json(json!({
    //     "message": "Move not found"
    // }))
}

#[post("/board/load/fen")]
pub async fn load_fen(
    fen_dto: web::Json<FenDTO>,
    board: web::Data<Mutex<Board>>, move_generator: web::Data<Mutex<MoveGenerator>>
) -> impl Responder {
    let board = &mut board.lock().unwrap();

    board.load_position(&fen_dto.fen);

    get_board_response(board, &mut move_generator.lock().unwrap())
}

pub fn get_board_response(board: &mut MutexGuard<'_, Board>, move_generator: &mut MutexGuard<'_, MoveGenerator>) -> HttpResponse {
    let fen = board.to_fen();
    let en_passant = board.get_en_passant_bb_position();
    let side_to_move = board.get_side_to_move();
    let zobrist = board.get_zobrist_hash();
    
    let pieces: Vec<Value> = get_pieces(board, move_generator);

    let black_captures = board.get_black_captures().iter().map(|capture| capture.to_fen(Color::White)).collect::<Vec<String>>();
    let white_captures = board.get_white_captures().iter().map(|capture| capture.to_fen(Color::Black)).collect::<Vec<String>>();

    HttpResponse::Ok().json(json!({
        "blackCaptures": black_captures,
        "enPassant": en_passant,
        "blackKingInCheck": false,
        "fen": fen,
        "whiteCaptures": white_captures,
        "siteToMove": side_to_move,
        "winner": board.get_winner(),
        "zobrit": zobrist,
        "pieces": pieces,
        "whiteKingInCheck": false,
        "movesHistory": board.get_moves_history().iter().map(|_move| MoveDTO::from_move(_move)).collect::<Vec<MoveDTO>>()
    }))
}

fn get_pieces(board: &mut Board, move_generator: &mut MoveGenerator) -> Vec<Value> {
    let moves: Vec<MoveDTO> = match move_generator.get_moves(board) {
        Ok(moves) => moves.iter().map(|_move| MoveDTO::from_move(_move)).collect(),
        Err(_) => Vec::new()
    };
    
    let mut pieces: Vec<Value> = Vec::with_capacity(64);

    for square in 0..64 {
        let piece_type = board.get_piece_type(square);
        let piece_color = board.get_piece_color(square);
        let fen = get_piece_letter(piece_color, &piece_type);

        pieces.push(json!({
            "color": piece_color,
            "fen": fen,
            "moves": moves.iter().filter(|_move| _move.from == square).collect::<Vec<_>>(),
            "position": square,
            "type": piece_type,
        }));
    }

    pieces
}