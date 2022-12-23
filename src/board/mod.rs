use std::{convert::TryFrom, fmt::Debug};

mod bitboard;
pub(in crate::board) mod board;
pub(in crate::board) mod constants;
mod error;
pub mod key;
pub mod r#move;
pub(in crate::board) mod types;
mod utils;

use bitboard::Bitboard;
use board::Board;
use constants::{BLACK_PIECES, DIRECTIONS, FILES, PIECE_VALUES, RANKS, WHITE_PIECES};
use error::{FENParsingError, NoPieceOnSquareError, SquareTakenError};
use r#move::Move;
use types::{CastlingRights, Color, Direction, Piece, Rank, Square};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardState {
    pub board: Board,
    pub side_to_move: Color,
    pub en_passant: Option<Square>,
    pub fifty_move_counter: usize,
    pub ply: usize,
    pub history_ply: usize,
    pub position_key: u64,
    pub castling_permissions: CastlingRights, // bits = [ wK, wQ, bK, bQ ]
}

impl BoardState {
    pub fn print_board(&self) {
        print!("{:?}", self.board)
    }

    pub fn from_fen(fen: impl ToString) -> Result<BoardState, FENParsingError> {
        let mut state = BoardState::default();

        let n_fields = 6;
        let fen_str = fen.to_string();
        if fen_str.split(' ').count() != n_fields {
            return Err(FENParsingError::new(" "));
        }
        let fields: Vec<String> = fen_str.split(' ').map(|s| s.to_string()).collect();

        // board
        state.board = Board::from_fen(&fields[0])?;

        // piece to move
        if fields[1] == "b".to_string() {
            state.side_to_move = Color::Black
        }

        // castling
        state.castling_permissions = CastlingRights::from_fen(&fields[2])?;

        // en passant
        state.en_passant = Square::from_fen(&fields[3])?;

        // half move clock
        match fields[4].parse::<usize>() {
            Ok(num) => {
                state.history_ply = num;
                state.ply = num;
            }
            Err(_) => return Err(FENParsingError::new("")),
        }

        // full move number discarded

        Ok(state)
    }

    pub fn make_move(self, _mv: Move) -> BoardState {
        todo!()
    }

    pub fn unmake_move(self, _mv: Move) -> BoardState {
        todo!()
    }

    pub fn legal_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = vec![];
        for piece in self.board.pieces(color) {
            let piece_squares: Vec<Square> = self.board.bitboard(piece).into();
            for sq in piece_squares {
                match piece {
                    Piece::WhitePawn => {
                        let fwd_mailbox_no = sq + Direction::North as i8;

                        // pawn start
                        if sq.rank() == Rank::Rank2 {
                            let sq_2_in_front =
                                Square::from_mailbox_no(sq + 2 * Direction::North as i8);
                            if (!self.board.sq_taken(Square::from_mailbox_no(fwd_mailbox_no)))
                                & (!self.board.sq_taken(sq_2_in_front))
                            {
                                moves.push(Move::new(
                                    sq,
                                    sq_2_in_front,
                                    None,
                                    None,
                                    false,
                                    true,
                                    false,
                                ));
                            }
                        }

                        // en passant

                        // normal forward & promotion
                        if fwd_mailbox_no >= 0 {
                            let fwd_sq = Square::from_mailbox_no(fwd_mailbox_no);
                            let promoted = if sq.rank() == Rank::Rank7 {
                                Some(Piece::WhiteQueen)
                            } else {
                                None
                            };
                            if !self.board.sq_taken(fwd_sq) {
                                moves.push(Move::new(
                                    sq, fwd_sq, None, promoted, false, false, false,
                                ));
                            }
                        }

                        // capture + capture promotion
                        let left_diag_mailbox_no = sq + Direction::NorthWest as i8;
                        if left_diag_mailbox_no >= 0 {
                            let ld_sq = Square::from_mailbox_no(left_diag_mailbox_no);
                            if self.board.sq_taken_by_color(ld_sq, Color::Black) {
                                let captured = self.board.piece(&ld_sq);
                                let promoted = if sq.rank() == Rank::Rank7 {
                                    Some(Piece::WhiteQueen)
                                } else {
                                    None
                                };
                                moves.push(Move::new(
                                    sq, ld_sq, captured, promoted, false, false, false,
                                ));
                            }
                        };
                        let right_diag_mailbox_no = sq + Direction::NorthEast as i8;
                        if right_diag_mailbox_no >= 0 {
                            let rd_sq = Square::from_mailbox_no(right_diag_mailbox_no);
                            if self.board.sq_taken_by_color(rd_sq, Color::Black) {
                                let captured = self.board.piece(&rd_sq);
                                let promoted = if sq.rank() == Rank::Rank7 {
                                    Some(Piece::WhiteQueen)
                                } else {
                                    None
                                };
                                moves.push(Move::new(
                                    sq, rd_sq, captured, promoted, false, false, false,
                                ));
                            }
                        }
                    }
                    Piece::BlackPawn => {
                        let fwd_mailbox_no = sq + Direction::South as i8;

                        // pawn start
                        if sq.rank() == Rank::Rank7 {
                            let sq_2_in_front =
                                Square::from_mailbox_no(sq + 2 * Direction::South as i8);
                            if (!self.board.sq_taken(Square::from_mailbox_no(fwd_mailbox_no)))
                                & (!self.board.sq_taken(sq_2_in_front))
                            {
                                moves.push(Move::new(
                                    sq,
                                    sq_2_in_front,
                                    None,
                                    None,
                                    false,
                                    true,
                                    false,
                                ));
                            }
                        }

                        // en passant

                        // normal forward & promotion
                        if fwd_mailbox_no >= 0 {
                            let fwd_sq = Square::from_mailbox_no(fwd_mailbox_no);
                            let promoted = if sq.rank() == Rank::Rank2 {
                                Some(Piece::BlackQueen)
                            } else {
                                None
                            };
                            if !self.board.sq_taken(fwd_sq) {
                                moves.push(Move::new(
                                    sq, fwd_sq, None, promoted, false, false, false,
                                ));
                            }
                        }

                        // capture + capture promotion
                        let left_diag_mailbox_no = sq + Direction::SouthWest as i8;
                        if left_diag_mailbox_no >= 0 {
                            let ld_sq = Square::from_mailbox_no(left_diag_mailbox_no);
                            if self.board.sq_taken_by_color(ld_sq, Color::White) {
                                let captured = self.board.piece(&ld_sq);
                                let promoted = if sq.rank() == Rank::Rank7 {
                                    Some(Piece::BlackQueen)
                                } else {
                                    None
                                };
                                moves.push(Move::new(
                                    sq, ld_sq, captured, promoted, false, false, false,
                                ));
                            }
                        };
                        let right_diag_mailbox_no = sq + Direction::SouthEast as i8;
                        if right_diag_mailbox_no >= 0 {
                            let rd_sq = Square::from_mailbox_no(right_diag_mailbox_no);
                            if self.board.sq_taken_by_color(rd_sq, Color::White) {
                                let captured = self.board.piece(&rd_sq);
                                let promoted = if sq.rank() == Rank::Rank2 {
                                    Some(Piece::BlackQueen)
                                } else {
                                    None
                                };
                                moves.push(Move::new(
                                    sq, rd_sq, captured, promoted, false, false, false,
                                ));
                            }
                        }
                    }
                    Piece::WhiteKnight | Piece::BlackKnight => {
                        let dirs = &DIRECTIONS[piece.attack_direction_idx()];
                        for dir in dirs {
                            let target_sq_mailbox_no = sq + *dir as i8;
                            if target_sq_mailbox_no >= 0 {
                                let other_sq = Square::from_mailbox_no(target_sq_mailbox_no);
                                if !self.board.sq_taken_by_color(other_sq, piece.color()) {
                                    let captured = self.board.piece(&other_sq);
                                    moves.push(Move::new(
                                        sq, other_sq, captured, None, false, false, false,
                                    ));
                                }
                            }
                        }
                    }
                    Piece::WhiteBishop
                    | Piece::BlackBishop
                    | Piece::WhiteRook
                    | Piece::BlackRook
                    | Piece::WhiteQueen
                    | Piece::BlackQueen => {
                        let dirs = &DIRECTIONS[piece.attack_direction_idx()].to_vec();
                        let color = piece.color();
                        utils::recur_move_search(&self.board, color, dirs, &mut moves, sq, 1);
                    }
                    Piece::WhiteKing | Piece::BlackKing => {
                        let dirs = &DIRECTIONS[piece.attack_direction_idx()];
                        for dir in dirs {
                            let target_sq_mailbox_no = sq + *dir as i8;
                            if target_sq_mailbox_no >= 0 {
                                let other_sq = Square::from_mailbox_no(target_sq_mailbox_no);
                                if !self.board.sq_taken_by_color(other_sq, piece.color())
                                    & !self
                                        .board
                                        .is_square_attacked(other_sq, piece.opposing_color())
                                {
                                    let captured = self.board.piece(&other_sq);
                                    moves.push(Move::new(
                                        sq, other_sq, captured, None, false, false, false,
                                    ));
                                }
                            }
                        }
                    }
                };
            }
        }
        moves
    }
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            board: Board::default(),
            side_to_move: Color::White,
            en_passant: None,
            fifty_move_counter: 0,
            ply: 0,
            history_ply: 0,
            position_key: 0,
            castling_permissions: CastlingRights(0b1111),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;
    use crate::board::constants::SQUARES;

    #[test]
    fn test_board_state_from_fen() {
        let start_state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed_state = BoardState::from_fen(start_state).unwrap();
        assert_eq!(parsed_state, BoardState::default());
    }

    #[test]
    fn test_board_state_legal_moves() {
        let fen = "5r2/1k1P1pP1/4q1BB/7n/6p1/1p2Q2b/PP2p3/1K3N2 b - - 0 1";
        let board_state = BoardState::from_fen(fen).unwrap();
        let mut white_moves = board_state.legal_moves(Color::White);
        let mut black_moves = board_state.legal_moves(Color::Black);

        let mut expected_white_moves = vec![
            // pawns
            Move::new(Square::A2, Square::A3, None, None, false, false, false),
            Move::new(Square::A2, Square::A4, None, None, false, true, false),
            Move::new(
                Square::A2,
                Square::B3,
                Some(Piece::BlackPawn),
                None,
                false,
                false,
                false,
            ),
            Move::new(
                Square::D7,
                Square::D8,
                None,
                Some(Piece::WhiteQueen),
                false,
                false,
                false,
            ),
            Move::new(
                Square::G7,
                Square::G8,
                None,
                Some(Piece::WhiteQueen),
                false,
                false,
                false,
            ),
            Move::new(
                Square::G7,
                Square::F8,
                Some(Piece::BlackRook),
                Some(Piece::WhiteQueen),
                false,
                false,
                false,
            ),
            // knights
            Move::new(Square::F1, Square::D2, None, None, false, false, false),
            Move::new(Square::F1, Square::G3, None, None, false, false, false),
            Move::new(Square::F1, Square::H2, None, None, false, false, false),
            // bishops
            Move::new(Square::G6, Square::H7, None, None, false, false, false),
            Move::new(
                Square::G6,
                Square::F7,
                Some(Piece::BlackPawn),
                None,
                false,
                false,
                false,
            ),
            Move::new(
                Square::G6,
                Square::H5,
                Some(Piece::BlackKnight),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::G6, Square::F5, None, None, false, false, false),
            Move::new(Square::G6, Square::E4, None, None, false, false, false),
            Move::new(Square::G6, Square::D3, None, None, false, false, false),
            Move::new(Square::G6, Square::C2, None, None, false, false, false),
            Move::new(Square::H6, Square::G5, None, None, false, false, false),
            Move::new(Square::H6, Square::F4, None, None, false, false, false),
            // queen
            Move::new(Square::E3, Square::F3, None, None, false, false, false),
            Move::new(Square::E3, Square::G3, None, None, false, false, false),
            Move::new(
                Square::E3,
                Square::H3,
                Some(Piece::BlackBishop),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::E3, Square::D3, None, None, false, false, false),
            Move::new(Square::E3, Square::C3, None, None, false, false, false),
            Move::new(
                Square::E3,
                Square::B3,
                Some(Piece::BlackPawn),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::E3, Square::D2, None, None, false, false, false),
            Move::new(Square::E3, Square::C1, None, None, false, false, false),
            Move::new(Square::E3, Square::F2, None, None, false, false, false),
            Move::new(Square::E3, Square::G1, None, None, false, false, false),
            Move::new(Square::E3, Square::F4, None, None, false, false, false),
            Move::new(Square::E3, Square::G5, None, None, false, false, false),
            Move::new(Square::E3, Square::D4, None, None, false, false, false),
            Move::new(Square::E3, Square::C5, None, None, false, false, false),
            Move::new(Square::E3, Square::B6, None, None, false, false, false),
            Move::new(Square::E3, Square::A7, None, None, false, false, false),
            Move::new(Square::E3, Square::E4, None, None, false, false, false),
            Move::new(Square::E3, Square::E5, None, None, false, false, false),
            Move::new(
                Square::E3,
                Square::E6,
                Some(Piece::BlackQueen),
                None,
                false,
                false,
                false,
            ),
            Move::new(
                Square::E3,
                Square::E2,
                Some(Piece::BlackPawn),
                None,
                false,
                false,
                false,
            ),
            // king
            Move::new(Square::B1, Square::A1, None, None, false, false, false),
            Move::new(Square::B1, Square::C1, None, None, false, false, false),
        ];

        let mut expected_black_moves = vec![
            // pawns
            Move::new(
                Square::B3,
                Square::A2,
                Some(Piece::WhitePawn),
                None,
                false,
                false,
                false,
            ),
            Move::new(
                Square::E2,
                Square::E1,
                None,
                Some(Piece::BlackQueen),
                false,
                false,
                false,
            ),
            Move::new(
                Square::E2,
                Square::F1,
                Some(Piece::WhiteKnight),
                Some(Piece::BlackQueen),
                false,
                false,
                false,
            ),
            Move::new(Square::F7, Square::F6, None, None, false, false, false),
            Move::new(Square::F7, Square::F5, None, None, false, true, false),
            Move::new(
                Square::F7,
                Square::G6,
                Some(Piece::WhiteBishop),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::G4, Square::G3, None, None, false, false, false),
            // knights
            Move::new(
                Square::H5,
                Square::G7,
                Some(Piece::WhitePawn),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::H5, Square::F6, None, None, false, false, false),
            Move::new(Square::H5, Square::F4, None, None, false, false, false),
            Move::new(Square::H5, Square::G3, None, None, false, false, false),
            // rooks
            Move::new(Square::F8, Square::E8, None, None, false, false, false),
            Move::new(Square::F8, Square::D8, None, None, false, false, false),
            Move::new(Square::F8, Square::C8, None, None, false, false, false),
            Move::new(Square::F8, Square::B8, None, None, false, false, false),
            Move::new(Square::F8, Square::A8, None, None, false, false, false),
            Move::new(Square::F8, Square::G8, None, None, false, false, false),
            Move::new(Square::F8, Square::H8, None, None, false, false, false),
            // bishops
            Move::new(Square::H3, Square::G2, None, None, false, false, false),
            Move::new(
                Square::H3,
                Square::F1,
                Some(Piece::WhiteKnight),
                None,
                false,
                false,
                false,
            ),
            // queen
            Move::new(Square::E6, Square::E7, None, None, false, false, false),
            Move::new(Square::E6, Square::E8, None, None, false, false, false),
            Move::new(Square::E6, Square::E5, None, None, false, false, false),
            Move::new(Square::E6, Square::E4, None, None, false, false, false),
            Move::new(
                Square::E6,
                Square::E3,
                Some(Piece::WhiteQueen),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::E6, Square::F6, None, None, false, false, false),
            Move::new(
                Square::E6,
                Square::G6,
                Some(Piece::WhiteBishop),
                None,
                false,
                false,
                false,
            ),
            Move::new(Square::E6, Square::D6, None, None, false, false, false),
            Move::new(Square::E6, Square::C6, None, None, false, false, false),
            Move::new(Square::E6, Square::B6, None, None, false, false, false),
            Move::new(Square::E6, Square::A6, None, None, false, false, false),
            Move::new(Square::E6, Square::F5, None, None, false, false, false),
            Move::new(Square::E6, Square::D5, None, None, false, false, false),
            Move::new(Square::E6, Square::C4, None, None, false, false, false),
            Move::new(
                Square::E6,
                Square::D7,
                Some(Piece::WhitePawn),
                None,
                false,
                false,
                false,
            ),
            // king
            Move::new(Square::B7, Square::B8, None, None, false, false, false),
            Move::new(Square::B7, Square::A8, None, None, false, false, false),
            Move::new(Square::B7, Square::A6, None, None, false, false, false),
            Move::new(Square::B7, Square::C6, None, None, false, false, false),
            Move::new(Square::B7, Square::C7, None, None, false, false, false),
        ];

        let (_, _) = (white_moves.sort(), expected_white_moves.sort());
        let (_, _) = (black_moves.sort(), expected_black_moves.sort());
        assert_eq!(white_moves, expected_white_moves);
        assert_eq!(black_moves, expected_black_moves);
    }
}
