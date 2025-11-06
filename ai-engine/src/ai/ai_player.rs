use core::f32;
use std::time::{Duration, Instant};

use crate::{constants::constants::WINNER_DRAW, game_bit_board::{_move::_move::Move, board::Board, enums::{Color, PieceType}, move_generator::move_generator::MoveGenerator}};

use super::{
    ai_utils::{get_board_value, get_sorted_moves},
    transposition_table::{EntryFlag, TranspositionTable, TranspositionTableEntry},
};

pub struct AIPlayer {
    transposition_table: TranspositionTable,
    time_to_think: u64,
    start_time: Instant,
    interruped: bool,
    states: u64
}

impl AIPlayer {
    pub fn new(time_to_think: u64) -> Self{
       Self {
        transposition_table: TranspositionTable::new(),
        time_to_think,
        start_time: Instant::now(),
        interruped: false,
        states: 0
       } 
    }

    pub fn get_ai_move(&mut self, board: &mut Board, move_generator: &mut MoveGenerator) -> (u128, u8, f32, u64, Option<Move>) {
        self.start_time = Instant::now();

        let mut depth = 5;

        let (value, best_move) = self.negamax(
            board,
            f32::MIN,
            f32::MAX,
            -1.0,
            depth,
            move_generator,
        );

        if best_move.is_some() {
            let best_move = best_move.unwrap();

            println!(
                "\nTransposition table size: ~{}kb. It got {} hits\n",
                self.transposition_table.estimated_memory_usage_kb(), self.transposition_table.get_hits()
            );

            println!("Evaluated {} states in {}ms with depth of {} and {} hits in the table. Best move eval: {}",
                 self.states, self.start_time.elapsed().as_millis(), depth,
                 self.transposition_table.get_hits(), best_move.get_move_worth());
        }

        (self.start_time.elapsed().as_millis(), depth, value, self.states, best_move)
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

        //if self._is_time_over() {
        //    self.interruped = true;
        //
        //    return f32::MIN;
        //}
        self.states += 1;

        let key = (board.get_zobrist_hash(), depth);

        let result = self.transposition_table.retrieve(key);

        if result.is_some() {
            let transposition_table_entry = result.unwrap();

            return (transposition_table_entry.value, transposition_table_entry.best_move);
        }

        if depth == 0 || board.is_game_finished() {
            let value = get_board_value(board, color) * color;

            self.transposition_table.store(
                key, 
                TranspositionTableEntry {
                    depth, value, best_move: None, flag: EntryFlag::Exact
                });

            return (value, None);
        }

        let mut best_move: Option<Move> = None;

        let mut moves: Vec<Move> = get_sorted_moves(board, color, move_generator);

        let mut value = f32::MIN;

        for _move in moves.iter_mut() {
            board.move_piece(_move);

            let result = -self.negamax(
                board,
                -beta,
                -alpha,
                -color,
                depth - 1,
                move_generator,
            ).0;

            if result > value {
                value = result;
                best_move = Some(*_move);
            }

            board.unmake_last_move();

            alpha = alpha.max(value);

            if alpha >= beta {
                break;
            }
        }

        self.transposition_table.store(
            key, 
            TranspositionTableEntry {
                depth, value, best_move, flag: EntryFlag::Exact
            });

        (value, best_move)
    }


    #[inline(always)]
    fn _is_time_over(&self) -> bool {
        Instant::now().duration_since(self.start_time) >= Duration::new(self.time_to_think, 0)
    }
}
