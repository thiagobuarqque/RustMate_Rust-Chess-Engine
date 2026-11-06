use crate::game_bit_board::{
    board::Board,
    enums::{Color, PieceType},
    utils::bitwise_utils::pop_lsb,
};

use super::zobrist_utils::{
    get_piece_index, BLACK_KING_SIDE_CASTLE_INDEX, BLACK_QUEEN_SIDE_CASTLE_INDEX,
    WHITE_KING_SIDE_CASTLE_INDEX, WHITE_QUEEN_SIDE_CASTLE_INDEX,
};

// https://www.chessprogramming.org/Zobrist_Hashing
//
// At program initialization, we generate an array of pseudorandom numbers:
// 1. One number for each piece at each square
// 2. One number to indicate the side to move is black
// 3. Four numbers to indicate the castling rights, though usually 16 (2^4) are
//    used for speed
// 4. Eight numbers to indicate the file of a valid En passant square, if any
//
// This leaves us with an array with 781 (12*64 + 1 + 4 + 8) random numbers.
// Since pawns don't happen on first and eighth rank, one might be fine with
// 12*64 though.

struct ZobristKeys {
    board_hashes: [[u64; 12]; 64],
    castling_rights_hash: [u64; 4],
    en_passant_hash: [u64; 8],
    side_to_move_hash: u64,
}

/// Generated at compile time, shared by every board
static KEYS: ZobristKeys = generate_keys(123);

/// https://prng.di.unimi.it/splitmix64.c
const fn split_mix_64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);

    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);

    z ^ (z >> 31)
}

const fn generate_keys(seed: u64) -> ZobristKeys {
    let mut state = seed;

    let mut board_hashes = [[0u64; 12]; 64];

    let mut square = 0;
    while square < 64 {
        let mut piece = 0;
        while piece < 12 {
            board_hashes[square][piece] = split_mix_64(&mut state);
            piece += 1;
        }
        square += 1;
    }

    let side_to_move_hash = split_mix_64(&mut state);

    // Castling rights
    let mut castling_rights_hash = [0u64; 4];
    let mut i = 0;
    while i < 4 {
        castling_rights_hash[i] = split_mix_64(&mut state);
        i += 1;
    }

    // En passant (8 possible files)
    let mut en_passant_hash = [0u64; 8];
    let mut i = 0;
    while i < 8 {
        en_passant_hash[i] = split_mix_64(&mut state);
        i += 1;
    }

    ZobristKeys {
        board_hashes,
        castling_rights_hash,
        en_passant_hash,
        side_to_move_hash,
    }
}

#[derive(Debug, Clone)]
pub struct Zobrist {
    hash: u64,
}

impl Zobrist {
    pub fn new() -> Self { Self { hash: 0 } }

    pub fn get_hash(&self) -> u64 { self.hash }

    pub fn set_hash(&mut self, hash: u64) { self.hash = hash }

    pub fn on_remove_piece(&mut self, piece: PieceType, color: Color, from_square: usize) {
        let moved_piece_index = get_piece_index(color, piece);

        self.hash ^= KEYS.board_hashes[from_square][moved_piece_index];
    }

    pub fn on_place_piece(&mut self, piece: PieceType, color: Color, to_square: usize) {
        let moved_piece_index = get_piece_index(color, piece);

        self.hash ^= KEYS.board_hashes[to_square][moved_piece_index];
    }

    pub fn on_after_move(&mut self) { self.hash ^= KEYS.side_to_move_hash; }

    pub fn update_en_passant_hash(&mut self, en_passant_square_bb: u64) {
        let square = pop_lsb(&mut (en_passant_square_bb.clone())) as usize;

        let file = square % 8;

        self.hash ^= KEYS.en_passant_hash[file];
    }

    pub fn update_hash_on_black_lose_king_side_castle(&mut self) {
        self.hash ^= KEYS.castling_rights_hash[BLACK_KING_SIDE_CASTLE_INDEX];
    }

    pub fn update_hash_on_black_lose_queen_side_castle(&mut self) {
        self.hash ^= KEYS.castling_rights_hash[BLACK_QUEEN_SIDE_CASTLE_INDEX];
    }

    pub fn update_hash_on_white_lose_king_side_castle(&mut self) {
        self.hash ^= KEYS.castling_rights_hash[WHITE_KING_SIDE_CASTLE_INDEX];
    }

    pub fn update_hash_on_white_lose_queen_side_castle(&mut self) {
        self.hash ^= KEYS.castling_rights_hash[WHITE_QUEEN_SIDE_CASTLE_INDEX];
    }

    pub fn compute_hash(&mut self, board: &Board) -> u64 {
        let mut hash = 0u64;

        if board.get_side_to_move().is_black() {
            hash ^= KEYS.side_to_move_hash;
        }

        if board.has_king_side_castle_right(Color::Black) {
            hash ^= KEYS.castling_rights_hash[BLACK_KING_SIDE_CASTLE_INDEX];
        }

        if board.has_queen_side_castle_right(Color::Black) {
            hash ^= KEYS.castling_rights_hash[BLACK_QUEEN_SIDE_CASTLE_INDEX];
        }

        if board.has_king_side_castle_right(Color::White) {
            hash ^= KEYS.castling_rights_hash[WHITE_KING_SIDE_CASTLE_INDEX];
        }

        if board.has_queen_side_castle_right(Color::White) {
            hash ^= KEYS.castling_rights_hash[WHITE_QUEEN_SIDE_CASTLE_INDEX];
        }

        let mut en_passant_square_bb = board.get_en_passant_piece_square_bb();

        if en_passant_square_bb != 0 {
            let square = pop_lsb(&mut en_passant_square_bb) as usize;

            let file = square % 8;

            hash ^= KEYS.en_passant_hash[file];
        }

        for square in 0..64_usize {
            let piece_type = board.get_piece_type(square);
            let piece_color = board.get_piece_color(square);

            if piece_type != PieceType::Empty {
                hash ^= KEYS.board_hashes[square][get_piece_index(piece_color, piece_type)];
            }
        }

        self.hash = hash;

        hash
    }
}
