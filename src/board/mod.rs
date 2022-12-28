use std::fmt::Debug;

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
use constants::DIRECTIONS;
use error::FENParsingError;
use r#move::{Move, MoveList};
use types::{CastlingRight, CastlingRights, Color, Direction, Piece, Rank, Square};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardState {
    pub board: Board,
    pub side_to_move: Color,
    pub en_passant: Option<Square>,
    pub fifty_move_counter: u8,
    pub ply: u8,
    pub history_ply: u8,
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
        match fields[4].parse::<u8>() {
            Ok(num) => {
                state.history_ply = num;
                state.ply = num;
            }
            Err(_) => return Err(FENParsingError::new("")),
        }

        // full move number discarded

        Ok(state)
    }

    pub fn make_moves<I>(self, moves: I) -> BoardState
    where
        I: Iterator<Item = Move>,
    {
        let mut state = self.clone();
        for mv in moves {
            state = state.make_move(mv);
        }
        state
    }

    // should the below two operate on self or &self?
    // can they return self / &self? does that change our alloc's?
    pub fn make_move(self, mv: Move) -> BoardState {
        let mut new_state = self.clone();
        if mv.captured().is_some() {
            new_state.board.remove_piece(mv.to_sq());
        }
        new_state.board.move_piece(mv.from_sq(), mv.to_sq());

        if mv.castle() {
            if mv.from_sq() == Square::E1 {
                if mv.to_sq() == Square::G1 {
                    new_state.board.move_piece(Square::H1, Square::F1).unwrap()
                } else if mv.to_sq() == Square::C1 {
                    new_state.board.move_piece(Square::A1, Square::D1).unwrap()
                }
                new_state.castling_permissions.0 -=
                    CastlingRight::WhiteKing as u8 + CastlingRight::WhiteQueen as u8;
            } else if mv.from_sq() == Square::E8 {
                if mv.to_sq() == Square::G8 {
                    new_state.board.move_piece(Square::H8, Square::F8).unwrap()
                } else if mv.to_sq() == Square::C8 {
                    new_state.board.move_piece(Square::A8, Square::D8).unwrap()
                }
                new_state.castling_permissions.0 -=
                    CastlingRight::BlackKing as u8 + CastlingRight::BlackQueen as u8;
            }
        }
        if (mv.from_sq() == Square::A1) & (self.board.piece(&Square::A1) == Some(Piece::WhiteRook))
        {
            new_state.castling_permissions.0 &=
                CastlingRights::all().0 - CastlingRight::WhiteQueen as u8;
        } else if (mv.from_sq() == Square::H1)
            & (self.board.piece(&Square::H1) == Some(Piece::WhiteRook))
        {
            new_state.castling_permissions.0 &=
                CastlingRights::all().0 - CastlingRight::WhiteKing as u8;
        } else if (mv.from_sq() == Square::A8)
            & (self.board.piece(&Square::A8) == Some(Piece::BlackRook))
        {
            new_state.castling_permissions.0 &=
                CastlingRights::all().0 - CastlingRight::BlackQueen as u8;
        } else if (mv.from_sq() == Square::H8)
            & (self.board.piece(&Square::H8) == Some(Piece::BlackRook))
        {
            new_state.castling_permissions.0 &=
                CastlingRights::all().0 - CastlingRight::BlackKing as u8;
        };

        if mv.pawn_start() {
            let dir = match self.side_to_move {
                Color::White => Direction::South,
                Color::Black => Direction::North,
            };
            let sq = Square::from_mailbox_no(mv.to_sq() + dir as i8);
            new_state.en_passant = Some(sq);
        } else {
            new_state.en_passant = None;
        }
        if mv.en_passant() {
            let dir = match self.side_to_move {
                Color::White => Direction::South, // white moving, capture black pawn on sq south of en passant sq
                Color::Black => Direction::North, // black moving, capture white pawn on sq north of en passant sq
            };
            let capture_sq = Square::from_mailbox_no(self.en_passant.unwrap() + dir as i8);
            let _ = new_state.board.remove_piece(capture_sq).unwrap();
        }

        // TODO: pos key

        if (mv.captured().is_some())
            | (self.board.piece(&mv.from_sq()) == Some(Piece::WhitePawn))
            | (self.board.piece(&mv.from_sq()) == Some(Piece::BlackPawn))
        {
            new_state.fifty_move_counter = 0;
        } else {
            new_state.fifty_move_counter += 1;
        }

        if let Some(piece) = mv.promoted() {
            new_state.board.remove_piece(mv.to_sq());
            new_state.board.add_piece(piece, mv.to_sq());
        }

        new_state.ply += 1;
        new_state.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        new_state
    }

    pub fn legal_moves(&self) -> MoveList {
        let color = self.side_to_move;
        let mut moves = MoveList::empty();
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
                            } else {
                                if let Some(ep_sq) = self.en_passant {
                                    if ld_sq == ep_sq {
                                        moves.push(Move::new(
                                            sq,
                                            ld_sq,
                                            Some(Piece::BlackPawn),
                                            None,
                                            true,
                                            false,
                                            false,
                                        ))
                                    }
                                }
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
                            } else {
                                if let Some(ep_sq) = self.en_passant {
                                    if rd_sq == ep_sq {
                                        moves.push(Move::new(
                                            sq,
                                            rd_sq,
                                            Some(Piece::BlackPawn),
                                            None,
                                            true,
                                            false,
                                            false,
                                        ))
                                    }
                                }
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
                            } else {
                                if let Some(ep_sq) = self.en_passant {
                                    if ld_sq == ep_sq {
                                        moves.push(Move::new(
                                            sq,
                                            ld_sq,
                                            Some(Piece::WhitePawn),
                                            None,
                                            true,
                                            false,
                                            false,
                                        ))
                                    }
                                }
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
                            } else {
                                if let Some(ep_sq) = self.en_passant {
                                    if rd_sq == ep_sq {
                                        moves.push(Move::new(
                                            sq,
                                            rd_sq,
                                            Some(Piece::WhitePawn),
                                            None,
                                            true,
                                            false,
                                            false,
                                        ))
                                    }
                                }
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
                        // normal moves
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
                        // castling
                        match piece.color() {
                            Color::White => {
                                if self.castling_permissions.white_kingside()
                                    & !self.board.sq_taken(Square::F1)
                                    & !self.board.sq_taken(Square::G1)
                                    & !self.board.is_square_attacked(Square::F1, Color::Black)
                                    & !self.board.is_square_attacked(Square::G1, Color::Black)
                                {
                                    moves.push(Move::new(
                                        Square::E1,
                                        Square::G1,
                                        None,
                                        None,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                                if self.castling_permissions.white_queenside()
                                    & !self.board.sq_taken(Square::C1)
                                    & !self.board.sq_taken(Square::D1)
                                    & !self.board.is_square_attacked(Square::C1, Color::Black)
                                    & !self.board.is_square_attacked(Square::D1, Color::Black)
                                {
                                    moves.push(Move::new(
                                        Square::E1,
                                        Square::C1,
                                        None,
                                        None,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                            }
                            Color::Black => {
                                if self.castling_permissions.black_kingside()
                                    & !self.board.sq_taken(Square::F8)
                                    & !self.board.sq_taken(Square::G8)
                                    & !self.board.is_square_attacked(Square::F8, Color::White)
                                    & !self.board.is_square_attacked(Square::G8, Color::White)
                                {
                                    moves.push(Move::new(
                                        Square::E8,
                                        Square::G8,
                                        None,
                                        None,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                                if self.castling_permissions.black_queenside()
                                    & !self.board.sq_taken(Square::C8)
                                    & !self.board.sq_taken(Square::D8)
                                    & !self.board.is_square_attacked(Square::C8, Color::White)
                                    & !self.board.is_square_attacked(Square::D8, Color::White)
                                {
                                    moves.push(Move::new(
                                        Square::E8,
                                        Square::C8,
                                        None,
                                        None,
                                        false,
                                        false,
                                        true,
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

    use crate::board::constants::PIECE_VALUES;

    use super::*;

    #[test]
    fn test_board_state_from_fen() {
        let start_state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed_state = BoardState::from_fen(start_state).unwrap();
        assert_eq!(parsed_state, BoardState::default());
    }

    #[test]
    fn test_board_state_make_move() {
        let ruy_lopez_opening = vec![
            Move::new(Square::E2, Square::E4, None, None, false, true, false),
            Move::new(Square::E7, Square::E5, None, None, false, true, false),
            Move::new(Square::G1, Square::F3, None, None, false, false, false),
            Move::new(Square::B8, Square::C6, None, None, false, false, false),
            Move::new(Square::F1, Square::B5, None, None, false, false, false),
        ];
        let mut state = BoardState::default();
        for mv in ruy_lopez_opening {
            state = state.make_move(mv);
        }
        assert_eq!(state.board.piece(&Square::B5), Some(Piece::WhiteBishop));
        assert!(state.en_passant.is_none());
        assert_eq!(state.fifty_move_counter, 3);
        assert_eq!(state.castling_permissions.0, 0b1111);

        let rook_moved_state = state.make_move(Move::new(
            Square::H1,
            Square::G1,
            None,
            None,
            false,
            false,
            false,
        ));
        assert_eq!(
            rook_moved_state.castling_permissions.0,
            CastlingRights::all().0 - CastlingRight::WhiteKing as u8
        );

        let captured_state = state.make_move(Move::new(
            Square::B5, Square::C6, Some(Piece::BlackKnight), None, false, false, false));

        assert_eq!(captured_state.board.piece(&Square::C6), Some(Piece::WhiteBishop));
    }

    #[test]
    fn test_board_state_make_move_en_passant() {
        let fen = "8/3k4/8/8/5p2/8/4P3/3K4 w - - 0 1";
        let board_state = BoardState::from_fen(fen).unwrap();

        let ep_state = board_state.make_move(Move::new(
            Square::E2,
            Square::E4,
            None,
            None,
            false,
            true,
            false,
        ));
        assert_eq!(ep_state.en_passant, Some(Square::E3));
        assert_eq!(
            ep_state.board.material(Color::White),
            PIECE_VALUES[Piece::WhiteKing as usize] + PIECE_VALUES[Piece::WhitePawn as usize]
        );
        assert!(ep_state.board.piece(&Square::E4).is_some());
        let captured_state = ep_state.make_move(Move::new(
            Square::F4,
            Square::E3,
            Some(Piece::WhitePawn),
            None,
            true,
            false,
            false,
        ));
        assert_eq!(
            captured_state.board.material(Color::White),
            PIECE_VALUES[Piece::WhiteKing as usize]
        );
        assert!(captured_state.board.piece(&Square::E4).is_none());
    }

    #[test]
    fn test_board_state_legal_moves_en_passant() {
        let fen = "8/3k4/8/8/5p2/8/4P3/3K4 w - - 0 1";
        let board_state = BoardState::from_fen(fen).unwrap();

        let ep_state = board_state.make_move(Move::new(
            Square::E2,
            Square::E4,
            None,
            None,
            false,
            true,
            false,
        ));
        assert_eq!(
            ep_state
                .legal_moves()
                .filter(|m| m.en_passant())
                .count(),
            1
        );
    }

    #[test]
    fn test_board_state_make_move_castling() {
        let fen = "8/8/8/2bk4/8/8/8/R3K2R w KQ - 0 1";
        let board_state = BoardState::from_fen(fen).unwrap();

        let white_ks_state = board_state.clone().make_move(Move::new(
            Square::E1,
            Square::G1,
            None,
            None,
            false,
            false,
            true,
        ));
        assert_eq!(
            white_ks_state.board.piece(&Square::F1),
            Some(Piece::WhiteRook)
        );
        assert_eq!(
            white_ks_state.board.piece(&Square::G1),
            Some(Piece::WhiteKing)
        );
        let white_qs_state = board_state.clone().make_move(Move::new(
            Square::E1,
            Square::C1,
            None,
            None,
            false,
            false,
            true,
        ));
        assert_eq!(
            white_qs_state.board.piece(&Square::D1),
            Some(Piece::WhiteRook)
        );
        assert_eq!(
            white_qs_state.board.piece(&Square::C1),
            Some(Piece::WhiteKing)
        );

        let fen = "r3k2r/8/8/2b5/8/8/8/2R1K3 b kq - 0 1";
        let board_state = BoardState::from_fen(fen).unwrap();

        let black_ks_state = board_state.clone().make_move(Move::new(
            Square::E8,
            Square::G8,
            None,
            None,
            false,
            false,
            true,
        ));
        assert_eq!(
            black_ks_state.board.piece(&Square::F8),
            Some(Piece::BlackRook)
        );
        assert_eq!(
            black_ks_state.board.piece(&Square::G8),
            Some(Piece::BlackKing)
        );
        let black_qs_state = board_state.clone().make_move(Move::new(
            Square::E8,
            Square::C8,
            None,
            None,
            false,
            false,
            true,
        ));
        assert_eq!(
            black_qs_state.board.piece(&Square::D8),
            Some(Piece::BlackRook)
        );
        assert_eq!(
            black_qs_state.board.piece(&Square::C8),
            Some(Piece::BlackKing)
        );
    }

    #[test]
    fn test_board_state_legal_moves_castling() {
        // white
        let fen = "8/8/8/2bk4/8/8/8/R3K2R w KQ - 0 1";
        let mut board_state = BoardState::from_fen(fen).unwrap();

        // kingside invalid bishop on B5 attacks, queenside valid
        assert_eq!(
            board_state
                .legal_moves()
                .filter(|m| m.castle())
                .count(),
            1
        );
        board_state.board.remove_piece(Square::C5);
        assert_eq!(
            board_state
                .legal_moves()
                .filter(|m| m.castle())
                .count(),
            2
        );

        // black
        // both sides valid
        let fen = "r3k2r/8/8/2b5/8/8/8/2R1K3 b kq - 0 1";
        let mut board_state = BoardState::from_fen(fen).unwrap();
        assert_eq!(
            board_state
                .legal_moves()
                .filter(|m| m.castle())
                .count(),
            2
        );
        // rm pinning bishop, now queenside travels through attack
        board_state.board.remove_piece(Square::C5);
        assert_eq!(
            board_state
                .legal_moves()
                .filter(|m| m.castle())
                .count(),
            1
        );
    }

    #[test]
    fn test_board_state_legal_moves() {
        let white_to_move_fen = "5r2/1k1P1pP1/4q1BB/7n/6p1/1p2Q2b/PP2p3/1K3N2 w - - 0 1";
        let black_to_move_fen = "5r2/1k1P1pP1/4q1BB/7n/6p1/1p2Q2b/PP2p3/1K3N2 b - - 0 1";
        let mut white_moves = BoardState::from_fen(white_to_move_fen).unwrap().legal_moves();
        let mut black_moves = BoardState::from_fen(black_to_move_fen).unwrap().legal_moves();

        let mut expected_white_moves = MoveList::new(vec![
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
        ]);

        let mut expected_black_moves = MoveList::new(vec![
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
        ]);

        assert_eq!(white_moves.sorted(), expected_white_moves.sorted());
        assert_eq!(black_moves.sorted(), expected_black_moves.sorted());
    }
}
