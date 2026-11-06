#![allow(dead_code)]
#![allow(warnings)]

use std::{io, num::ParseIntError, sync::Mutex};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use ai::ai_player::AIPlayer;
use api::{ai_controller, board_controller};
use game_bit_board::{
    _move::_move::Move, board::Board, move_generator::move_generator::MoveGenerator,
};

mod ai;
mod api;
#[cfg(test)]
mod benchmarks;
mod constants;
mod dto;
mod game_bit_board;
mod global_state;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

fn to_usize(value: &str) -> Result<usize, ParseIntError> { value.parse() }

fn get_piece_attacks(from: usize, moves: &Vec<Move>) -> Vec<usize> {
    let mut attacks: Vec<usize> = Vec::new();

    for _move in moves {
        if _move.get_from() == from {
            attacks.push(_move.get_to());
        }
    }

    attacks
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    println!("Server started successfully 🚀!");

    let board = Data::new(Mutex::new(Board::new()));
    let move_generator = Data::new(Mutex::new(MoveGenerator::new()));
    // Kept between requests so its transposition table is reused
    let ai_player = Data::new(Mutex::new(AIPlayer::new(3)));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_any_origin()
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(Data::clone(&board))
            .app_data(Data::clone(&move_generator))
            .app_data(Data::clone(&ai_player))
            .service(board_controller::get_board)
            .service(board_controller::get_move_generation_count)
            .service(board_controller::load_fen)
            .service(board_controller::move_piece)
            .service(ai_controller::ai_move)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
