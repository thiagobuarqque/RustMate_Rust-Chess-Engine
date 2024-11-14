use core::f32;
use std::time::{Duration, Instant};

// use rayon::{iter::IntoParallelRefIterator, prelude::ParallelIterator};

// use crate::{
//     common::{contants::INVALID_BOARD_POSITION, enums::PieceColor, piece::Piece, piece_move::PieceMove},
//     game::board::Board,
// };

use crate::{constants::constants::WINNER_DRAW, game_bit_board::{_move::_move::Move, board::Board, enums::{Color, PieceType}, move_generator::move_generator::MoveGenerator}};

use super::{
    ai_utils::{get_board_value, get_sorted_moves},
    transposition_table::{EntryFlag, TranspositionTable, TranspositionTableEntry},
};

pub fn get_ai_move(board: &mut Board, move_generator: &mut MoveGenerator, time_to_think: u64) -> (u128, u8, f32, Move) {
    let mut depth = 1;
    let mut _move = Move::from_to(99, 99, Color::Black, PieceType::Empty);

    let mut transposition_table: TranspositionTable = TranspositionTable::new();
    
    let mut score = f32::MIN;

    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < Duration::new(time_to_think, 0)
        && depth < u8::MAX {

        let result = negamax(
            board,
            f32::MIN,
            f32::MAX,
            1.0,
            depth,
            &mut transposition_table,
            move_generator,
            start_time,
            time_to_think
        );

        // score = result.0;

        // if result.1.is_some() {
        //   _move = result.1.unwrap();
        // }

        println!("\nAt depth {depth} the best move is: {} and the evaluation is {}",
        result.1.unwrap(), result.0);


        if result.0 > score {
            println!("That move is better than the previous one: {_move}");
            score = result.0;
            _move = result.1.unwrap();
        }

        depth += 1;
    }

    println!(
        "Transposition table size: ~{}kb",
        transposition_table.estimated_memory_usage_kb()
    );

    // println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}", 
    //     transposition_table.len(), start_time.elapsed().as_millis(), depth, transposition_table.get_hits(), _move.get_move_worth());

    (start_time.elapsed().as_millis(), depth, score, _move)
}

fn negamax(
    board: &mut Board,
    mut alpha: f32,
    mut beta: f32,
    color: f32,
    depth: u8,
    transposition_table: &mut TranspositionTable,
    move_generator: &mut MoveGenerator,
    start_time: Instant,
    time_to_think: u64
) -> (f32, Option<Move>) {

    let mut best_move = None;
    let original_alpha = alpha;

    if let Some(entry) = transposition_table.retrieve((board.get_zobrist_hash(), depth)) {
        if entry.depth >= depth {
          best_move = entry.best_move.clone();
          
          if entry.flag == EntryFlag::Exact {
              return (entry.value, best_move);
          } else if entry.flag == EntryFlag::Lowerbound {
              alpha = alpha.max(entry.value);    
          } else if entry.flag == EntryFlag::Upperbound {
              beta = beta.min(entry.value);
          }
  
          if alpha >= beta {
              return (entry.value, best_move);
          }
        }
    }

    let mut moves: Vec<Move> = get_sorted_moves(&best_move, board, color, move_generator);

    let mut value = f32::MIN;

    if depth == 0 || board.is_game_finished() {
        let value: f32 = get_board_value(board, color);

        // Favors checkmates that require less moves
        return if board.is_game_finished() && board.get_winner() != WINNER_DRAW && depth > 1 {
            ((value * depth as f32) * color, best_move)
        } else {
            (value * color, best_move)
        };
    }

    for _move in moves.iter_mut() {
        if Instant::now().duration_since(start_time) >= Duration::new(time_to_think, 0) {
            break;
        }

        board.move_piece(_move);

        let score = -negamax(
            board,
            -beta,
            -alpha,
            -color,
            depth - 1,
            transposition_table,
            move_generator,
            start_time,
            time_to_think
        ).0;

        board.unmake_last_move();

        alpha = alpha.max(value);

        if score > value {
            value = score;
            best_move = Some(_move.clone());
        }

        if alpha >= beta {
            break;
        }
    }

    // println!("Best move at depth {depth}: {}", best_move.unwrap());

    transposition_table.store(
        (board.get_zobrist_hash(), depth),
        TranspositionTableEntry {
            depth,
            value,
            best_move,
            flag: if value <= original_alpha {
                EntryFlag::Upperbound         
            } else if value >= beta {
                EntryFlag::Lowerbound
            } else { EntryFlag::Exact }
        },
    );

    (value, best_move)
}
