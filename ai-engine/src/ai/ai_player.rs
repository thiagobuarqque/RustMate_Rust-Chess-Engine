use std::time::{Duration, Instant};

// use rayon::{iter::IntoParallelRefIterator, prelude::ParallelIterator};

// use crate::{
//     common::{contants::INVALID_BOARD_POSITION, enums::PieceColor, piece::Piece, piece_move::PieceMove},
//     game::board::Board,
// };

use crate::{constants::constants::WINNER_DRAW, game_bit_board::{_move::_move::Move, board::Board, enums::{Color, PieceType}, move_generator::move_generator::MoveGenerator}};

use super::{
    ai_utils::{get_board_value, get_sorted_moves},
    transposition_table::{TranspositionTable, TranspositionTableEntry},
};

pub fn get_ai_move(board: &mut Board, move_generator: &mut MoveGenerator, time_to_think: u64) -> (u128, u8, f32, Move) {
    let mut depth = 1;
    let mut best_move = Move::from_to(0, 0, Color::Black, PieceType::Empty);

    let mut transposition_table: TranspositionTable = TranspositionTable::new();
    
    let mut max = true;
    
    let mut alpha = f32::MIN;

    let start_time = Instant::now();

    // let (score, best_move) = negamax(
    //     board,
    //     -f32::MAX,
    //     -alpha,
    //     !max,
    //     depth,
    //     &mut transposition_table,
    //     move_generator,
    //     start_time,
    //     time_to_think
    // );

    while Instant::now().duration_since(start_time) < Duration::new(time_to_think, 0)
        && depth < u8::MAX {

        // let moves: Vec<Move> =
        //     get_sorted_moves(&Some(best_move), board, true, move_generator);

        let result = negamax(
            board,
            -f32::MAX,
            -alpha,
            max,
            depth,
            &mut transposition_table,
            move_generator,
            start_time,
            time_to_think
        );

        if result.0 > alpha {
            alpha = result.0;
            best_move = result.1.unwrap();
        }

        depth += 1;
        // max = !max;

    }
    println!(
        "Transposition table size: ~{}kb",
        transposition_table.estimated_memory_usage_kb()
    );

    println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}", 
        transposition_table.len(), start_time.elapsed().as_millis(), depth, transposition_table.get_hits(), best_move.get_move_worth());

    (start_time.elapsed().as_millis(), depth, alpha, best_move)
}

fn negamax(
    board: &mut Board,
    alpha: f32,
    beta: f32,
    max: bool,
    depth: u8,
    transposition_table: &mut TranspositionTable,
    move_generator: &mut MoveGenerator,
    start_time: Instant,
    time_to_think: u64
) -> (f32, Option<Move>) {

    // Check if the position is already in the table

    let mut best_move = None;

    if let Some(entry) = transposition_table.retrieve((board.get_zobrist_hash(), depth)) {
        best_move = entry.best_move.clone();

        if entry.depth >= depth {
            return (entry.value, best_move);
        }
    }

    let mut moves: Vec<Move> = get_sorted_moves(&best_move, board, max, move_generator);

    if depth == 0 || board.is_game_finished() {
        let value: f32 = get_board_value(board, max);

        transposition_table.store(
            (board.get_zobrist_hash(), depth),
            TranspositionTableEntry {
                depth,
                value,
                best_move,
            },
        );

        // Favors checkmates that require less moves
        return if board.is_game_finished() && depth > 1 {
            (value * depth as f32, best_move)
        } else {
            (value, best_move)
        };
    }

    let mut alpha = alpha;
    for _move in moves.iter_mut() {
        if Instant::now().duration_since(start_time) >= Duration::new(time_to_think, 0) {
            break;
        }

        board.move_piece(_move);

        // // Considering the move sorting is good: we could decrease
        // // the depth from the 5th move forward.
        // if i >= 4 && !_move.is_capture() && depth >= 2 {
        //     new_depth = 1;
        // }

        let score = -negamax(
            board,
            -beta,
            -alpha,
            !max,
            depth - 1,
            transposition_table,
            move_generator,
            start_time,
            time_to_think
        ).0;

        let game_finished = board.is_game_finished();

        let draw = board.get_winner() == WINNER_DRAW;

        board.unmake_last_move();

        if score > alpha {
            alpha = score;

            best_move = Some(_move.clone());

            if alpha >= beta {
                break;
            }
        }

        if game_finished && !draw {
            break;
        }
    }

    transposition_table.store(
        (board.get_zobrist_hash(), depth),
        TranspositionTableEntry {
            depth,
            value: alpha,
            best_move,
        },
    );

    (alpha, best_move)
}
