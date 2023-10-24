use super::{
    board::{bitboard::Bitboard, Board},
    constants::{BLACK_PIECES, DIRECTIONS, WHITE_PIECES},
    error::FENParsingError,
    move_gen::MoveGenerator,
    r#move::MoveList,
    types::{CastlingRights, Color, Piece, PieceType, Square},
    utils,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Position {
    pub board: Board,
    pub side_to_move: Color,
    pub en_passant: Option<Square>,
    pub castling_permissions: CastlingRights, // bits = [ wK, wQ, bK, bQ ]
    pub castling_perms_history: Vec<CastlingRights>,
    pub checkers: [Bitboard; 2],
    pub blockers_for_king: [Bitboard; 2],
    pub pinners: [Bitboard; 2],
    pub check_squares: [Bitboard; 8],
}

impl Default for Position {
    fn default() -> Self {
        Self {
            board: Board::default(),
            side_to_move: Color::White,
            en_passant: None,
            castling_permissions: CastlingRights::all(),
            castling_perms_history: vec![],
            // TODO: all the logic updating the below needs to be implemented
            checkers: [Bitboard::empty(); 2],
            blockers_for_king: [Bitboard::empty(); 2],
            pinners: [Bitboard::empty(); 2],
            check_squares: [Bitboard::empty(); 8],
        }
    }
}

impl Position {
    pub fn from_fields(fields: Vec<String>) -> Result<Position, FENParsingError> {
        let n_req_fields = 4;
        if fields.len() != n_req_fields {
            return Err(FENParsingError::new(format!(
                "Incorrect number of fields to parse position. Expected {}, got {}",
                n_req_fields,
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

    pub fn legal_moves(&self) -> MoveList {
        let color = self.side_to_move;
        let mut moves = MoveList::empty();

        for piece in self.board.pieces(color) {
            for sq in self.board.bitboard(piece) {
                if piece.piece_type() == PieceType::Pawn {
                    MoveGenerator::generate_pawn_moves(&self, piece, sq, &mut moves);
                } else {
                    MoveGenerator::generate_moves(&self, piece, sq, &mut moves);
                }
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
            let attack_bb = self.board.bitboard(*piece);

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
