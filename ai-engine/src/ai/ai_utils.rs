use crate::{constants::constants::WINNER_DRAW, game_bit_board::{_move::_move::Move, board::Board, enums::{Color, PieceType}, move_generator::move_generator::MoveGenerator, utils::bitwise_utils::pop_lsb}};

use super::{constants::{
    BISHOP_WORTH, BLACK_BISHOP_SQUARE_TABLE, BLACK_KING_SQUARE_TABLE_END_GAME, BLACK_KING_SQUARE_TABLE_MIDDLE_GAME, BLACK_KNIGHT_SQUARE_TABLE, BLACK_PAWN_SQUARE_TABLE, BLACK_ROOK_SQUARE_TABLE, END_GAME_PIECES_THRESHOLD, KING_WORTH, PAWN_WORTH, QUEEN_SQUARE_TABLE, QUEEN_WORTH, ROOK_WORTH, WHITE_BISHOP_SQUARE_TABLE, WHITE_KING_SQUARE_TABLE_END_GAME, WHITE_KING_SQUARE_TABLE_MIDDLE_GAME, WHITE_KNIGHT_SQUARE_TABLE, WHITE_PAWN_SQUARE_TABLE, WHITE_ROOK_SQUARE_TABLE
}, utils::get_piece_worth};

pub fn get_sorted_moves(
    best_move: &Option<Move>,
    board: &mut Board,
    max: bool,
    move_generator: &mut MoveGenerator
) -> Vec<Move> {
    let (mut moves, attack_data) = move_generator.get_moves(board).unwrap();

    let end_game = is_end_game(board);

    moves.iter_mut().for_each(|_move| {
        if _move.is_capture() {
            let target_piece = board.get_piece_type(_move.get_to());

            _move.sum_to_move_worth(
                (get_piece_worth(target_piece)) - get_piece_worth(_move.piece_type),
            );
        }

        if _move.is_promotion() {
            _move.sum_to_move_worth(get_piece_worth(_move.get_promotion_piece_type()) * 5.0);
        }

        let attacked_squares = !attack_data.king_allowed_squares;

        if attacked_squares & _move.get_to() as u64 != 0 {
            _move.sum_to_move_worth(-get_piece_worth(_move.piece_type))
        }

        _move.sum_to_move_worth(get_position_value(
            end_game,
            _move.piece_type,
            _move.get_from(),
            _move.color.is_white()
        ));

        if end_game && _move.piece_type == PieceType::King {
            _move.sum_to_move_worth(get_end_game_move_worth(board, max, _move));
        }
    });

    // TODO order also based on the hashmap with previous generated states

    if max {
        moves.sort_by(|a, b| b.move_worth.partial_cmp(&a.move_worth).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        moves.sort_by(|a, b| a.move_worth.partial_cmp(&b.move_worth).unwrap_or(std::cmp::Ordering::Equal));
    }

    if best_move.is_some() {
        let best_move = best_move.clone().unwrap();

        for (i, _move) in moves.iter().enumerate() {
            if _move.eq(&best_move) {
                moves.remove(i);
                moves.insert(0, best_move);

                break;
            }
        }
    }

    moves
}

fn get_end_game_move_worth(board: &mut Board, max: bool, _move: &Move) -> f32 {
    let (friendly_king_position, opponent_king_position) =
        if _move.color.is_white() {
            (
                _move.get_to(),
                board.get_king_position(Color::Black),
            )
        } else {
            (
                _move.get_to(),
                board.get_king_position(Color::White),
            )
        };

    let mut evaluation = 0.0_f32;

    let opponent_king_rank = (opponent_king_position / 8) as f32;
    let opponent_king_file = (opponent_king_position % 8) as f32;

    let opponent_king_dst_to_center_file = (3.0 - opponent_king_file).max(opponent_king_file - 4.0);
    let opponent_king_dst_to_center_rank = (3.0 - opponent_king_rank).max(opponent_king_rank - 4.0);

    let opponent_king_dst_from_center =
        opponent_king_dst_to_center_file + opponent_king_dst_to_center_rank;

    evaluation += opponent_king_dst_from_center as f32;

    let friendly_king_rank = (friendly_king_position / 8) as f32;
    let friendly_king_file = (friendly_king_position % 8) as f32;

    let dst_between_kings_file = (friendly_king_file - opponent_king_file).abs();
    let dst_between_kings_rank = (friendly_king_rank - opponent_king_rank).abs();

    let dst_between_kings = dst_between_kings_file + dst_between_kings_rank;

    evaluation += 14.0 - (dst_between_kings as f32);

    return evaluation
        * 10.0
        * calculate_end_game_weight(board)
        * if max { 1.0 } else { -1.0 };
}

// fn get_friendly_moves_and_attacked_positions(
//     pieces: &[Piece],
//     board: &Board,
// ) -> (Vec<Move>, Vec<i8>) {
//     let mut moves: Vec<Move> = pieces
//         .iter()
//         .filter(|piece| piece.is_white() == board.is_white_move())
//         .flat_map(|piece| piece.get_moves_clone())
//         .collect();
//     let attacked_positions: Vec<i8> = pieces
//         .iter()
//         .filter(|piece| {
//             piece.is_white() != board.is_white_move()
//                 && get_piece_type(piece.get_value()) != PieceType::Pawn
//         })
//         .flat_map(|piece| piece.get_moves_reference())
//         .map(|_move| _move.get_to_position())
//         .collect();
//     let mut promotion_moves: Vec<Move> = Vec::with_capacity(4);
//     moves
//         .iter()
//         .filter(|_move| _move.is_promotion())
//         .for_each(|_move| {
//             for promotion_option in get_promotion_options(is_white_piece(_move.get_piece_value())) {
//                 let mut move_clone = _move.clone();
//                 move_clone.set_promotion_value(promotion_option);
//                 promotion_moves.push(move_clone)
//             }
//         });
//     if !promotion_moves.is_empty() {
//         moves.retain(|_move| !_move.is_promotion());
//         moves.extend(promotion_moves);
//     }
//     (moves, attacked_positions)
// }

fn get_position_value(end_game: bool, piece_type: PieceType, position: usize, white: bool) -> f32 {
    if piece_type == PieceType::King {
        if end_game {
            return if white {
                WHITE_KING_SQUARE_TABLE_END_GAME[position]
            } else {
                BLACK_KING_SQUARE_TABLE_END_GAME[position]
            };
        }

        return if white {
            WHITE_KING_SQUARE_TABLE_MIDDLE_GAME[position]
        } else {
            BLACK_KING_SQUARE_TABLE_MIDDLE_GAME[position]
        };
    }

    if white {
        match piece_type {
            PieceType::Bishop => WHITE_BISHOP_SQUARE_TABLE[position],
            PieceType::Knight => WHITE_KNIGHT_SQUARE_TABLE[position],
            PieceType::Pawn => WHITE_PAWN_SQUARE_TABLE[position],
            PieceType::Queen => QUEEN_SQUARE_TABLE[position],
            PieceType::Rook => WHITE_ROOK_SQUARE_TABLE[position],
            _ => 0.0,
        }
    } else {
        match piece_type {
            PieceType::Bishop => BLACK_BISHOP_SQUARE_TABLE[position],
            PieceType::Knight => BLACK_KNIGHT_SQUARE_TABLE[position],
            PieceType::Pawn => BLACK_PAWN_SQUARE_TABLE[position],
            PieceType::Queen => QUEEN_SQUARE_TABLE[position],
            PieceType::Rook => BLACK_ROOK_SQUARE_TABLE[position],
            _ => 0.0,
        }
    }
}

fn is_end_game(board: &mut Board) -> bool {
    calculate_end_game_weight(board) >= 1.0
}

fn calculate_end_game_weight(board: &mut Board) -> f32 {
    let black_pieces = board.get_player_pieces_positions(Color::Black);
    let white_pieces = board.get_player_pieces_positions(Color::White);

    (END_GAME_PIECES_THRESHOLD / black_pieces.count_ones() as f32)
        .max(END_GAME_PIECES_THRESHOLD / white_pieces.count_ones() as f32)
}

pub fn get_board_value(board: &mut Board, max: bool) -> f32 {
    if board.is_game_finished() && board.get_winner() == WINNER_DRAW {
        // Draw
        return 0.0;
    } 
    
    if board.is_game_finished() {
        return KING_WORTH * 10.0 * if max {
            1.0
        } else {
            -1.0
        };
    }

    // The evaluation
    // f(p) = 200(K-K') -> always 0 since the two kings are always present
    //         + 9(Q-Q')
    //         + 5(R-R')
    //         + 3(B-B' + N-N')
    //         + 1(P-P')
    //         - 0.5(D-D' + S-S' + I-I')
    //         + 0.1(M-M') + ...
    //
    // ' means the opponent score
    // KQRBNP = number of kings, queens, rooks, bishops, knights and pawns
    // D,S,I = doubled, blocked and isolated pawns
    // M = Mobility (the number of legal moves)

    // let mut k: f32 = 0.0;
    let mut q: f32 = 0.0;
    let mut r: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut n: f32 = 0.0;
    let mut p: f32 = 0.0;

    let d: f32 = 0.0;
    let s: f32 = 0.0;
    let i: f32 = 0.0;
    let m: f32 = 0.0;

    let mut pst_score: f32 = 0.0;

    let end_game = is_end_game(board);

    let mut occupied_squares = board.get_occupied_squares();

    while occupied_squares != 0 {
        let square = pop_lsb(&mut occupied_squares) as usize;
        let color = board.get_piece_color(square);
        let piece_type = board.get_piece_type(square);

        let pst_value = get_position_value(
            end_game,
            piece_type,
            square,
            color.is_white()
        );

        let factor: f32 = if color.is_white() == board.get_side_to_move().is_white() {
            1.0
        } else {
            -1.0
        };

        pst_score = pst_value * factor;

        match piece_type {
            PieceType::Queen => q += factor,
            PieceType::Rook => r += factor,
            PieceType::Bishop => b += factor,
            PieceType::Knight => n += factor,
            PieceType::Pawn => {
                p += factor;

                // if is_doubled_pawn(board_state, square, color.is_white()) {
                //     d += factor;
                // }

                // if is_blocked_pawn(board_state, square, color.is_white()) {
                //     s += factor;
                // }

                // if is_isolated_pawn(board_state, square, color.is_white()) {
                //     i += factor;
                // }
            }
            // Additional cases for D, S, I, and M are handled below
            _ => (),
        }

        // Sum the for the amount of moves a play has
        // m += piece.get_moves_reference().len() as f32 * factor;
    }

    let score = (QUEEN_WORTH * q) + (ROOK_WORTH * r) + (BISHOP_WORTH * (b + n)) + (PAWN_WORTH * p)
        - ((d + s + i) / 2.0)
        + (m / 10.0)
        + pst_score;

    score * if max { 1.0 } else { -1.0 }
}

// pub fn is_isolated_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
//     let positions = [
//         get_adjacent_position(position, position - 1),
//         get_adjacent_position(position, position + 1),
//         get_adjacent_position(position, position - 9),
//         get_adjacent_position(position, position - 8),
//         get_adjacent_position(position, position - 7),
//         get_adjacent_position(position, position + 7),
//         get_adjacent_position(position, position + 8),
//         get_adjacent_position(position, position + 9),
//     ];

//     for adjacent_position in positions {
//         if !board_state.is_valid_position(adjacent_position) {
//             continue;
//         }

//         let piece = board_state.get_piece(adjacent_position);

//         if piece == EMPTY_PIECE {
//             continue;
//         }

//         if get_piece_type(piece) == PieceType::Pawn && is_white_piece(piece) == white_piece {
//             return false;
//         }
//     }

//     true
// }

// pub fn is_blocked_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
//     let offset: i8 = if white_piece { -8 } else { 8 };

//     let frontal_piece = board_state.get_piece(position + offset);

//     if get_piece_type(frontal_piece) != PieceType::Empty
//     // && white_piece != is_white_piece(frontal_piece)
//     {
//         let mut diagonal_left = 0;
//         let mut diagonal_right = 0;

//         if position % 8 != 0 {
//             let diagonal_offset = if white_piece { -1 } else { 1 };

//             diagonal_left = board_state.get_piece(position + offset + diagonal_offset);
//         }

//         if (position + 1) % 8 != 0 {
//             let diagonal_offset = if white_piece { 1 } else { -1 };

//             diagonal_right = board_state.get_piece(position + offset + diagonal_offset);
//         }

//         let diagonal_left_color = is_white_piece(diagonal_left);
//         let diagonal_right_color = is_white_piece(diagonal_right);

//         if diagonal_left == 0 && diagonal_right == 0 {
//             return true;
//         } else if diagonal_left != 0 && diagonal_right == 0 {
//             return diagonal_left_color == white_piece;
//         } else if diagonal_right != 0 && diagonal_left == 0 {
//             return diagonal_right_color == white_piece;
//         }

//         return diagonal_left_color == white_piece && diagonal_right_color == white_piece;
//     }

//     false
// }

// pub fn is_doubled_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
//     let offset: i8 = if white_piece { -8 } else { 8 };

//     let mut position: i8 = position + offset;
//     while board_state.is_valid_position(position) {
//         let frontal_piece = board_state.get_piece(position);

//         position += offset;

//         if frontal_piece == EMPTY_PIECE {
//             continue;
//         }

//         let piece_type = get_piece_type(frontal_piece);

//         if piece_type != PieceType::Pawn {
//             return false;
//         } else if white_piece == is_white_piece(frontal_piece) {
//             return true;
//         }
//     }

//     false
// }
