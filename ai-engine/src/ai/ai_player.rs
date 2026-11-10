use std::time::{Duration, Instant};

use crate::{
    constants::constants::WINNER_DRAW,
    game_bit_board::{
        _move::_move::Move,
        board::Board,
        enums::{Color, PieceType},
        move_generator::move_generator::MoveGenerator,
    },
};

use super::{
    ai_utils::{get_board_value, get_sorted_moves},
    constants::MATE_SCORE,
    transposition_table::{EntryFlag, TranspositionTable, TranspositionTableEntry},
};

/// Deepest search a time-limited player tries
const MAX_DEPTH: u8 = 64;

/// The clock is checked once every this many states, checking it on every
/// state is too slow
const TIME_CHECK_INTERVAL: u64 = 1024;

pub struct AIPlayer {
    transposition_table: TranspositionTable,
    time_to_think: Duration,
    max_depth: u8,
    root_depth: u8,
    start_time: Instant,
    interruped: bool,
    states: u64,
}

impl AIPlayer {
    /// Searches deeper and deeper until `time_to_think` seconds are over
    pub fn new(time_to_think: u64) -> Self {
        Self::with_limits(Duration::from_secs(time_to_think), MAX_DEPTH)
    }

    /// Searches up to `depth` without a time limit, so results are
    /// reproducible
    pub fn with_fixed_depth(depth: u8) -> Self { Self::with_limits(Duration::MAX, depth) }

    fn with_limits(time_to_think: Duration, max_depth: u8) -> Self {
        Self {
            transposition_table: TranspositionTable::new(),
            time_to_think,
            max_depth,
            root_depth: 0,
            start_time: Instant::now(),
            interruped: false,
            states: 0,
        }
    }

    pub fn get_ai_move(
        &mut self, board: &mut Board, move_generator: &mut MoveGenerator,
    ) -> (u128, u8, f32, u64, Option<Move>) {
        self.start_time = Instant::now();
        self.states = 0;
        self.interruped = false;
        self.transposition_table.reset_hits();

        let mut depth = 0;
        let mut value = 0.0;
        let mut best_move = None;

        // Iterative deepening: only the result of the deepest completed search
        // is used
        for search_depth in 1..=self.max_depth {
            // The next search takes longer than all the previous ones, it would not
            // finish in the remaining time
            if search_depth > 1 && self.start_time.elapsed() >= self.time_to_think / 2 {
                break;
            }

            self.root_depth = search_depth;

            let (search_value, search_move) =
                self.negamax(board, f32::MIN, f32::MAX, search_depth, move_generator);

            if self.interruped {
                break;
            }

            depth = search_depth;
            value = search_value;
            best_move = search_move;

            // Game over, or a forced mate that a deeper search can't change
            if best_move.is_none() || value.abs() >= MATE_SCORE {
                break;
            }
        }

        if best_move.is_some() {
            let best_move = best_move.unwrap();

            println!(
                "\nTransposition table size: ~{}kb. It got {} hits\n",
                self.transposition_table.estimated_memory_usage_kb(),
                self.transposition_table.get_hits()
            );

            println!(
                "Evaluated {} states in {}ms with depth of {} and {} table hits.",
                self.states,
                self.start_time.elapsed().as_millis(),
                depth,
                self.transposition_table.get_hits()
            );

            println!("Best move: {} with eval {}", best_move, best_move.get_move_worth());
        }

        (self.start_time.elapsed().as_millis(), depth, value, self.states, best_move)
    }

    fn negamax(
        &mut self, board: &mut Board, mut alpha: f32, mut beta: f32, depth: u8,
        move_generator: &mut MoveGenerator,
    ) -> (f32, Option<Move>) {
        self.states += 1;

        // The depth 1 search is never interrupted, so there is always a move
        if self.root_depth > 1 && self.states % TIME_CHECK_INTERVAL == 0 && self.is_time_over() {
            self.interruped = true;
        }

        // Unwind without storing anything, the search is incomplete
        if self.interruped {
            return (0.0, None);
        }

        let key = board.get_zobrist_hash();

        let alpha_original = alpha;

        let mut table_move = None;

        if let Some(entry) = self.transposition_table.retrieve(key) {
            // Even from a shallower search, the best move is likely still the best
            table_move = entry.best_move;

            // Only a search at least as deep as this one can be reused
            if entry.depth >= depth {
                match entry.flag {
                    EntryFlag::Exact => return (entry.value, entry.best_move),
                    EntryFlag::Lowerbound => alpha = alpha.max(entry.value),
                    EntryFlag::Upperbound => beta = beta.min(entry.value),
                }

                if alpha >= beta {
                    return (entry.value, entry.best_move);
                }
            }
        }

        if depth == 0 || board.is_game_finished() {
            // The evaluation is relative to the side to move
            let value = get_board_value(board, 1.0);

            self.transposition_table.store(TranspositionTableEntry {
                key,
                depth,
                value,
                best_move: None,
                flag: EntryFlag::Exact,
            });

            return (value, None);
        }

        let mut best_move: Option<Move> = None;

        let mut moves: Vec<Move> = get_sorted_moves(board, move_generator);

        // No legal moves, the move generator has set the winner
        if moves.is_empty() {
            let value = if board.get_winner() == WINNER_DRAW {
                0.0
            } else {
                // The more depth is left, the sooner the mate, the worse it is
                -(MATE_SCORE + depth as f32)
            };

            self.transposition_table.store(TranspositionTableEntry {
                key,
                depth,
                value,
                best_move: None,
                flag: EntryFlag::Exact,
            });

            return (value, None);
        }

        // Trying the best move first gives more and earlier cutoffs
        if let Some(table_move) = table_move {
            let index = moves
                .iter()
                .position(|_move| _move.as_u64() == table_move.as_u64());

            if let Some(index) = index {
                moves[..=index].rotate_right(1);
            }
        }

        let mut value = f32::MIN;

        for _move in moves.iter_mut() {
            board.move_piece(_move);

            let result = -self
                .negamax(board, -beta, -alpha, depth - 1, move_generator)
                .0;

            if result > value {
                value = result;
                best_move = Some(*_move);
            }

            board.unmake_last_move();

            if self.interruped {
                return (0.0, None);
            }

            alpha = alpha.max(value);

            if alpha >= beta {
                break;
            }
        }

        // A value outside the original window is only a bound, the search was
        // cut short there
        let flag = if value <= alpha_original {
            EntryFlag::Upperbound
        } else if value >= beta {
            EntryFlag::Lowerbound
        } else {
            EntryFlag::Exact
        };

        self.transposition_table.store(TranspositionTableEntry {
            key,
            depth,
            value,
            best_move,
            flag,
        });

        (value, best_move)
    }

    fn is_time_over(&self) -> bool { self.start_time.elapsed() >= self.time_to_think }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::constants::{INITIAL_FEN, WINNER_WHITE},
        game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator},
    };

    use super::AIPlayer;

    fn get_ai_move(fen: &str) -> (Board, MoveGenerator, String) {
        let mut board = Board::from_fen(fen);
        let mut move_generator = MoveGenerator::new();

        let (_, _, _, _, mut best_move) =
            AIPlayer::with_fixed_depth(5).get_ai_move(&mut board, &mut move_generator);

        let mut best_move = best_move.take().expect("The AI should always return a move");

        board.move_piece(&mut best_move);

        (board, move_generator, best_move.to_simple_notation())
    }

    #[test]
    fn test_avoids_stalemate_when_winning() {
        // Qb6 stalemates black
        let (_, _, best_move) = get_ai_move("k7/8/8/1Q6/8/8/8/7K w - - 0 1");

        assert_ne!("b5b6", best_move);
    }

    #[test]
    fn test_finds_mate_in_one() {
        // Qg8 is mate
        let (mut board, mut move_generator, best_move) =
            get_ai_move("k7/8/1K6/8/8/8/8/6Q1 w - - 0 1");

        let (moves, _) = move_generator.get_moves(&mut board).unwrap();

        assert!(moves.is_empty(), "{best_move} is not mate");
        assert_eq!(WINNER_WHITE, board.get_winner(), "{best_move} is not mate");
    }

    #[test]
    fn test_reuses_previous_search() {
        let mut board = Board::from_fen(INITIAL_FEN);
        let mut move_generator = MoveGenerator::new();
        let mut ai_player = AIPlayer::with_fixed_depth(5);

        let (_, _, _, first_states, first_move) =
            ai_player.get_ai_move(&mut board, &mut move_generator);
        let (_, _, _, second_states, second_move) =
            ai_player.get_ai_move(&mut board, &mut move_generator);

        assert!(second_states < first_states);
        assert_eq!(
            first_move.unwrap().to_simple_notation(),
            second_move.unwrap().to_simple_notation()
        );
    }

    #[test]
    fn test_returns_a_move_without_time() {
        let mut board = Board::from_fen(INITIAL_FEN);
        let mut move_generator = MoveGenerator::new();

        let (_, depth, _, _, best_move) =
            AIPlayer::new(0).get_ai_move(&mut board, &mut move_generator);

        assert_eq!(1, depth);
        assert!(best_move.is_some());
    }

    #[test]
    fn test_stops_when_time_is_up() {
        let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let mut board = Board::from_fen(fen);
        let mut move_generator = MoveGenerator::new();

        let (duration, depth, _, _, best_move) =
            AIPlayer::new(1).get_ai_move(&mut board, &mut move_generator);

        assert!(duration <= 1_250, "Took {duration}ms");
        assert!(depth >= 1);
        assert!(best_move.is_some());
    }
}
