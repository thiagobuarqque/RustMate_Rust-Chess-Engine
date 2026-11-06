//! Performance benchmarks. Ignored by default; run them in release mode:
//!
//! `cargo test --release benchmarks -- --ignored --nocapture --test-threads=1`

use std::time::{Duration, Instant};

use crate::{
    ai::ai_player::AIPlayer,
    game_bit_board::{board::Board, move_generator::move_generator::MoveGenerator},
};

const RUNS: usize = 5;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
const MIDDLE_GAME_FEN: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

/// Plain perft, without the transposition lookup used by `count_moves`, so it
/// measures raw move generation + make/unmake speed.
fn perft(board: &mut Board, depth: usize, move_generator: &mut MoveGenerator) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut moves = move_generator.get_moves(board).unwrap().0;

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;

    for _move in moves.iter_mut() {
        board.move_piece(_move);
        nodes += perft(board, depth - 1, move_generator);
        board.unmake_last_move();
    }

    nodes
}

/// Runs `f` with a fresh value from `setup` each time, only `f` is timed
fn best_of<S, T>(
    runs: usize, mut setup: impl FnMut() -> S, mut f: impl FnMut(S) -> T,
) -> (Duration, T) {
    let mut best: Option<(Duration, T)> = None;

    for _ in 0..runs {
        let value = setup();
        let start = Instant::now();
        let result = f(value);
        let elapsed = start.elapsed();

        if best
            .as_ref()
            .map_or(true, |(duration, _)| elapsed < *duration)
        {
            best = Some((elapsed, result));
        }
    }

    best.unwrap()
}

fn bench_perft(name: &str, fen: &str, depth: usize, expected_nodes: u64) {
    let mut move_generator = MoveGenerator::new();
    let mut board = Board::from_fen(fen);

    let (elapsed, nodes) = best_of(RUNS, || (), |_| perft(&mut board, depth, &mut move_generator));

    assert_eq!(expected_nodes, nodes);

    println!(
        "[perft] {name} depth {depth}: {nodes} nodes in {}ms ({:.2} Mnps)",
        elapsed.as_millis(),
        nodes as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
}

/// Searches with a fresh `AIPlayer` each run, so the transposition table does
/// not leak between runs. The expected states/move/eval pin the search
/// behaviour: performance changes must not alter them.
fn bench_search(name: &str, fen: &str, expected: (u64, &str, f32)) {
    let mut move_generator = MoveGenerator::new();
    let mut board = Board::from_fen(fen);

    let (elapsed, (depth, eval, states, best_move)) = best_of(
        RUNS,
        || AIPlayer::new(0),
        |mut ai_player| {
            let (_, depth, eval, states, best_move) =
                ai_player.get_ai_move(&mut board, &mut move_generator);

            (depth, eval, states, best_move.unwrap().to_simple_notation())
        },
    );

    println!(
        "[search] {name} depth {depth}: {states} states in {}ms ({:.0} knps), best move {best_move}, eval {eval}",
        elapsed.as_millis(),
        states as f64 / elapsed.as_secs_f64() / 1_000.0
    );

    assert_eq!(expected, (states, best_move.as_str(), eval));
}

#[test]
#[ignore]
fn bench_perft_start_position() {
    bench_perft("start", START_FEN, 6, 119_060_324);
}

#[test]
#[ignore]
fn bench_perft_kiwipete() {
    bench_perft("kiwipete", KIWIPETE_FEN, 5, 193_690_690);
}

#[test]
#[ignore]
fn bench_search_start_position() {
    bench_search("start", START_FEN, (74_183, "e2e4", 131.0));
}

#[test]
#[ignore]
fn bench_search_kiwipete() {
    bench_search("kiwipete", KIWIPETE_FEN, (121_009, "d5e6", 359.0));
}

#[test]
#[ignore]
fn bench_search_middle_game() {
    bench_search("middle game", MIDDLE_GAME_FEN, (447_376, "c3d5", 263.0));
}
