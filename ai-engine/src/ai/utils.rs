use crate::game_bit_board::enums::PieceType;

use super::constants::{
    BISHOP_WORTH, KING_WORTH, KNIGHT_WORTH, PAWN_WORTH, QUEEN_WORTH, ROOK_WORTH,
};

#[inline(always)]
pub fn get_piece_worth(piece_type: PieceType) -> f32 {
    match piece_type {
        PieceType::Bishop => BISHOP_WORTH,
        PieceType::King => KING_WORTH,
        PieceType::Knight => KNIGHT_WORTH,
        PieceType::Pawn => PAWN_WORTH,
        PieceType::Queen => QUEEN_WORTH,
        PieceType::Rook => ROOK_WORTH,
        PieceType::Empty => 0.0,
    }
}
