use super::{
    board::{bitboard::Bitboard, Board},
    constants::{BLACK_PIECES, DIRECTIONS, WHITE_PIECES},
    error::FENParsingError,
    r#move::{Move, MoveList},
    types::{CastlingRights, Color, Direction, Piece, Rank, Square},
    utils,
};

// TODO: needs info about check and what not
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Position {
    pub board: Board,
    pub side_to_move: Color,
    pub en_passant: Option<Square>,
    pub castling_permissions: CastlingRights, // bits = [ wK, wQ, bK, bQ ]
    pub castling_perms_history: Vec<CastlingRights>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            board: Board::default(),
            side_to_move: Color::White,
            en_passant: None,
            castling_permissions: CastlingRights::all(),
            castling_perms_history: vec![],
        }
    }
}

impl Position {
    pub fn from_fields(fields: Vec<String>) -> Result<Position, FENParsingError> {
        let n_req_fields = 4;
        if fields.len() != n_req_fields {
            return Err(FENParsingError::new(format!(
                "Incorrect number of fields to parse position. Expected 4, got {}",
                fields.len()
            )));
        }

        let mut pos = Position::default();

        // board
        pos.board = Board::from_fen(&fields[0])?;

        // piece to move
        if fields[1] == "b".to_string() {
            pos.side_to_move = Color::Black
        }

        // castling
        pos.castling_permissions = CastlingRights::from_fen(&fields[2])?;

        // en passant
        pos.en_passant = Square::from_fen(&fields[3])?;

        Ok(pos)
    }

    ///
    ///
    pub fn from_fen(fen: impl ToString) -> Result<Position, FENParsingError> {
        let fields: Vec<String> = fen.to_string().split(' ').map(|s| s.to_string()).collect();
        Position::from_fields(fields)
    }

    pub fn flip_side(&mut self) {
        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    ///
    ///
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
                                    & !self.is_square_attacked(other_sq, piece.opposing_color())
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
                                    & !self.is_square_attacked(Square::F1, Color::Black)
                                    & !self.is_square_attacked(Square::G1, Color::Black)
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
                                    & !self.is_square_attacked(Square::C1, Color::Black)
                                    & !self.is_square_attacked(Square::D1, Color::Black)
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
                                    & !self.is_square_attacked(Square::F8, Color::White)
                                    & !self.is_square_attacked(Square::G8, Color::White)
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
                                    & !self.is_square_attacked(Square::C8, Color::White)
                                    & !self.is_square_attacked(Square::D8, Color::White)
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

    ///
    ///
    pub fn is_square_attacked(&self, sq: Square, color: Color) -> bool {
        let piece_array: &[Piece; 6] = match color {
            Color::White => &WHITE_PIECES,
            Color::Black => &BLACK_PIECES,
        };
        for piece in piece_array {
            let attack_dirs = DIRECTIONS[piece.attack_direction_idx()].to_vec();
            let attack_bb = self.board.bitboard(piece);

            if attack_bb.0 == 0x0 {
                continue;
            }

            if !piece.can_slide() {
                for dir in attack_dirs {
                    let mailbox_no = sq + dir as i8;
                    if mailbox_no >= 0 {
                        let other_sq_bb: Bitboard = Square::from_mailbox_no(mailbox_no).into();
                        if (other_sq_bb & attack_bb).0 != 0x0 {
                            return true;
                        }
                    }
                }
            } else {
                if utils::recur_attack_sq_search(&self.board, attack_dirs, sq, 1, attack_bb) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use crate::play::constants::SQUARES;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_from_fields() {
        let ok_fields: Vec<String> = vec!["8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8", "b", "-", "-"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(Position::from_fields(ok_fields).is_ok());
        let err_fields: Vec<String> = vec!["8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8", "b"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(Position::from_fields(err_fields).is_err());
    }

    #[test]
    fn test_from_fen() {
        let ok_fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - -";
        assert!(Position::from_fen(ok_fen).is_ok());
        let err_fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b";
        assert!(Position::from_fen(err_fen).is_err());
    }

    #[test]
    fn test_legal_moves() {}

    // #[test]
    // fn test_board_state_legal_moves_pins_are_illegal() {
    //     let state = State::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    //     let invalid_mv = Move::new(
    //         Square::B5,
    //         Square::B6,
    //         None,
    //         None,
    //         false,
    //         false,
    //         false
    //     );
    //     assert_eq!(state.legal_moves().filter(|mv| *mv == invalid_mv).count(), 0);

    //     // find another example (pinning a different piece besides a pawn)
    // }

    #[test]
    fn test_is_square_attacked() {
        let fen = "rn1qkb1r/pp2pppp/3p3n/2p2b2/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQ -";
        let pos = Position::from_fen(fen).unwrap();

        let squares_attacked_by_white = HashSet::from([
            Square::B1, // rook on a1
            Square::C1, // rook on a1
            Square::D1, // knight on c3 + king on e1
            Square::E1, // king on e1
            Square::F1, // king on e1 + rook on h1
            Square::G1, // rook on h1 + knight on f3
            Square::A2, // rook on a1
            Square::B2, // bishop on c1
            Square::C2, // queen on d1
            Square::D2, // queen on d1, king on e1, knight on d2
            Square::E2, // knight on c3, queen on d1, bishop on f1
            Square::F2, // king on e1
            Square::G2, // bishop on f1
            Square::H2, // rook on h1
            Square::A3, // pawn on b2
            Square::B3, // pawn on a2
            Square::C3, // pawns on b2 + d2
            Square::D3, // pawn on c2 + bishop on f1
            Square::E3, // pawns on d2 + f2
            Square::F3, // queen on d1 + pawn on g2
            Square::G3, // pawns on f2 + h2
            Square::H3, // pawn on g2
            Square::A4, // knight on c3
            Square::C4, // bishop on f1
            Square::D4, // knight on f3
            Square::E4, // knight on c3
            Square::H4, // knight on f3
            Square::B5, // bishop on f1
            Square::D5, // pawn on e4 + knight on c3
            Square::E5, // knight on f3
            Square::F5, // pawn on e4
            Square::G5, // knight on f3
            Square::A6, // bishop on f1
        ]);
        for sq in &squares_attacked_by_white {
            assert!(pos.is_square_attacked(*sq, Color::White));
        }
        let squares_not_attacked_by_white = SQUARES
            .iter()
            .filter(|sq| !squares_attacked_by_white.contains(sq));
        for sq in squares_not_attacked_by_white {
            assert!(!pos.is_square_attacked(*sq, Color::White));
        }

        let squares_attacked_by_black = HashSet::from([
            Square::B8, // rook on a8
            Square::C8, // queen on d8 + bishop on f5
            Square::D8, // king on e8
            Square::E8, // queen on d8
            Square::F8, // king on e8 + rook on h8
            Square::G8, // knight on h6
            Square::A7, // rook on a8
            Square::C7, // queen on d8
            Square::D7, // queen on d8, bishop on f5, king on e8
            Square::E7, // queen on d8, king on e8, bishop on f8
            Square::F7, // king on e8 + knight on h6
            Square::G7, // bishop on f8
            Square::H7, // rook on h8 + bishop on f5
            Square::A6, // pawn on b7 + knight on b8
            Square::B6, // pawn on a7 + queen on d8
            Square::C6, // knight on b8
            Square::D6, // queen on d8 + pawn on e7
            Square::E6, // pawn on f7, bishop on f5
            Square::F6, // pawns on e7, g7
            Square::G6, // pawns on f7, h7, bishop on f5
            Square::H6, // pawn on g7
            Square::A5, // queen on d8
            Square::C5, // pawn on d6
            Square::E5, // pawn on d6
            Square::F5, // knight on h6
            Square::B4, // pawn on c5
            Square::D4, // pawn on c5
            Square::E4, // bishop on f5
            Square::G4, // bishop on f5 + knight on h6
            Square::H3, // bishop on f5
        ]);
        for sq in &squares_attacked_by_black {
            assert!(pos.is_square_attacked(*sq, Color::Black));
        }
        let squares_not_attacked_by_black = SQUARES
            .iter()
            .filter(|sq| !squares_attacked_by_black.contains(sq));
        for sq in squares_not_attacked_by_black {
            assert!(!pos.is_square_attacked(*sq, Color::Black));
        }
    }
}
