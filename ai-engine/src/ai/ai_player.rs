use core::f32;
use std::time::{Duration, Instant};

// use rayon::{iter::IntoParallelRefIterator, prelude::ParallelIterator};

// use crate::{
//     common::{contants::INVALID_BOARD_POSITION, enums::PieceColor, piece::Piece, piece_move::PieceMove},
//     game::board::Board,
// };

use crate::{ai::tree_node::TreeNode, constants::constants::WINNER_DRAW, game_bit_board::{_move::_move::Move, board::Board, enums::{Color, PieceType}, move_generator::move_generator::MoveGenerator}};

use super::{
    ai_utils::{get_board_value, get_sorted_moves},
    transposition_table::{EntryFlag, TranspositionTable, TranspositionTableEntry},
};

pub struct AIPlayer {
    transposition_table: TranspositionTable,
    time_to_think: u64,
    start_time: Instant,
    interruped: bool,
}

impl AIPlayer {
    pub fn new(time_to_think: u64) -> Self{
       Self {
        transposition_table: TranspositionTable::new(),
        time_to_think,
        start_time: Instant::now(),
        interruped: false
       } 
    }

    pub fn get_ai_move(&mut self, board: &mut Board, move_generator: &mut MoveGenerator) -> (u128, u8, f32, Move) {
        let mut depth = 1;
        let mut _move = Move::from_to(99, 99, Color::Black, PieceType::Empty);

        let mut score = f32::MIN;

        self.start_time = Instant::now();

        while depth < u8::MAX {
            let result = self.negamax(
                board,
                f32::MIN,
                f32::MAX,
                1.0,
                depth,
                move_generator,
            );

            if self.interruped {
                break;
            }

            score = result.0;

            if result.1.is_some() {
                _move = result.1.unwrap();
            }

            depth += 1;
        }

        println!(
            "\nTransposition table size: ~{}kb. It got {} hits\n",
            self.transposition_table.estimated_memory_usage_kb(), self.transposition_table.get_hits()
        );

        // println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}", 
        //     transposition_table.len(), start_time.elapsed().as_millis(), depth, transposition_table.get_hits(), _move.get_move_worth());

        (self.start_time.elapsed().as_millis(), depth, score, _move)
    }

    fn negamax(
        &mut self,
        board: &mut Board,
        mut alpha: f32,
        mut beta: f32,
        color: f32,
        depth: u8,
        move_generator: &mut MoveGenerator,
    ) -> (f32, Option<Move>) {

        if Instant::now().duration_since(self.start_time) >= Duration::new(self.time_to_think, 0) {
            self.interruped = true;

            return (f32::MIN, None);
        }

        let mut best_move = None;

        let original_alpha = alpha;

        if let Some(entry) = self.transposition_table.retrieve((board.get_zobrist_hash(), depth)) {
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

        let mut moves: Vec<Move> = get_sorted_moves(&best_move, board, color, move_generator);

        let mut value = f32::MIN;

        if depth == 0 || board.is_game_finished() {
            let value: f32 = get_board_value(board, color);
            
            // Favors checkmates that require less moves
            // let value = if board.is_game_finished() && board.get_winner() != WINNER_DRAW && depth > 1 {
            //     (value * depth as f32) * color
            // } else {
            //     value * color
            // };
            let value = value * color;

            // println!("It's a leaf! Board eval: {}", value);

            self.transposition_table.store(
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

            return (value, best_move);
        }

        for _move in moves.iter_mut() {
            board.move_piece(_move);

            let score = -self.negamax(
                board,
                -beta,
                -alpha,
                -color,
                depth - 1,
                move_generator,
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

        self.transposition_table.store(
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
}
