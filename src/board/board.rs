use std::{convert::TryFrom, fmt};

use crate::board::{
    bitboard::Bitboard,
    constants::{BLACK_PIECES, DIRECTIONS, FILES, PIECE_VALUES, RANKS, WHITE_PIECES},
    error::{FENParsingError, NoPieceOnSquareError, SquareTakenError},
    types::{Color, Piece, Rank, Square},
    utils,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    white_pawns: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_rooks: Bitboard,
    white_queens: Bitboard,
    white_king: Bitboard,
    black_pawns: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_rooks: Bitboard,
    black_queens: Bitboard,
    black_king: Bitboard,
}

impl Board {
    pub fn empty() -> Self {
        Board {
            white_pawns: Bitboard::empty(),
            white_knights: Bitboard::empty(),
            white_bishops: Bitboard::empty(),
            white_rooks: Bitboard::empty(),
            white_queens: Bitboard::empty(),
            white_king: Bitboard::empty(),
            black_pawns: Bitboard::empty(),
            black_bishops: Bitboard::empty(),
            black_knights: Bitboard::empty(),
            black_rooks: Bitboard::empty(),
            black_queens: Bitboard::empty(),
            black_king: Bitboard::empty(),
        }
    }

    fn bitboard_union(&self) -> Bitboard {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
            | self.black_pawns
            | self.black_bishops
            | self.black_knights
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn bitboard(&self, piece: &Piece) -> Bitboard {
        match piece {
            Piece::WhitePawn => self.white_pawns,
            Piece::WhiteKnight => self.white_knights,
            Piece::WhiteBishop => self.white_bishops,
            Piece::WhiteRook => self.white_rooks,
            Piece::WhiteQueen => self.white_queens,
            Piece::WhiteKing => self.white_king,
            Piece::BlackPawn => self.black_pawns,
            Piece::BlackKnight => self.black_knights,
            Piece::BlackBishop => self.black_bishops,
            Piece::BlackRook => self.black_rooks,
            Piece::BlackQueen => self.black_queens,
            Piece::BlackKing => self.black_king,
        }
    }

    pub fn sq_taken(&self, sq: Square) -> bool {
        (self.bitboard_union() & sq.into()).0 != 0x0
    }

    pub fn sq_taken_by_color(&self, sq: Square, color: Color) -> bool {
        let bb = match color {
            Color::White => {
                self.white_pawns
                    | self.white_knights
                    | self.white_bishops
                    | self.white_rooks
                    | self.white_queens
                    | self.white_king
            }
            Color::Black => {
                self.black_pawns
                    | self.black_bishops
                    | self.black_knights
                    | self.black_rooks
                    | self.black_queens
                    | self.black_king
            }
        };
        (bb & sq.into()).0 != 0x0
    }

    pub fn add_piece(&mut self, piece: Piece, sq: Square) -> Result<(), SquareTakenError> {
        let sq_bb: Bitboard = sq.into();
        if self.sq_taken(sq) {
            Err(SquareTakenError::new(sq))
        } else {
            match piece {
                Piece::WhitePawn => self.white_pawns |= sq_bb,
                Piece::WhiteKnight => self.white_knights |= sq_bb,
                Piece::WhiteBishop => self.white_bishops |= sq_bb,
                Piece::WhiteRook => self.white_rooks |= sq_bb,
                Piece::WhiteQueen => self.white_queens |= sq_bb,
                Piece::WhiteKing => self.white_king |= sq_bb, // should validate if a king exists?
                Piece::BlackPawn => self.black_pawns |= sq_bb,
                Piece::BlackKnight => self.black_knights |= sq_bb,
                Piece::BlackBishop => self.black_bishops |= sq_bb,
                Piece::BlackRook => self.black_rooks |= sq_bb,
                Piece::BlackQueen => self.black_queens |= sq_bb,
                Piece::BlackKing => self.black_king |= sq_bb,
            }
            Ok(())
        }
    }

    pub fn remove_piece(&mut self, sq: Square) -> Result<Piece, NoPieceOnSquareError> {
        let sq_bb: Bitboard = sq.into();
        match self.piece(&sq) {
            Some(piece) => {
                match piece {
                    Piece::WhitePawn => self.white_pawns ^= sq_bb,
                    Piece::WhiteKnight => self.white_knights ^= sq_bb,
                    Piece::WhiteBishop => self.white_bishops ^= sq_bb,
                    Piece::WhiteRook => self.white_rooks ^= sq_bb,
                    Piece::WhiteQueen => self.white_queens ^= sq_bb,
                    Piece::WhiteKing => self.white_king ^= sq_bb,
                    Piece::BlackPawn => self.black_pawns ^= sq_bb,
                    Piece::BlackKnight => self.black_knights ^= sq_bb,
                    Piece::BlackBishop => self.black_bishops ^= sq_bb,
                    Piece::BlackRook => self.black_rooks ^= sq_bb,
                    Piece::BlackQueen => self.black_queens ^= sq_bb,
                    Piece::BlackKing => self.black_king ^= sq_bb,
                }
                Ok(piece)
            }
            None => Err(NoPieceOnSquareError::new(sq)),
        }
    }

    pub fn replace_piece(&mut self, sq: Square, piece: Piece) -> Result<(), NoPieceOnSquareError> {
        if !self.sq_taken(sq) {
            return Err(NoPieceOnSquareError::new(sq));
        };
        self.remove_piece(sq);
        self.add_piece(piece, sq);
        Ok(())
    }

    pub fn move_piece(&mut self, origin: Square, dest: Square) -> Result<(), SquareTakenError> {
        if self.sq_taken(dest) {
            return Err(SquareTakenError::new(dest));
        };
        let piece = self.piece(&origin).unwrap();
        self.remove_piece(origin);
        self.add_piece(piece, dest);
        Ok(())
    }

    pub fn pieces(&self, color: Color) -> Vec<&Piece> {
        let mut v = vec![];
        let piece_arr = match color {
            Color::White => &WHITE_PIECES,
            Color::Black => &BLACK_PIECES,
        };
        for piece in piece_arr {
            if self.bitboard(&piece).0 != 0x0 {
                v.push(piece)
            }
        }
        v
    }

    pub fn material(&self, color: Color) -> u32 {
        match color {
            Color::White => {
                self.white_pawns.pop_count() * PIECE_VALUES[Piece::WhitePawn as usize]
                    + self.white_knights.pop_count() * PIECE_VALUES[Piece::WhiteKnight as usize]
                    + self.white_bishops.pop_count() * PIECE_VALUES[Piece::WhiteBishop as usize]
                    + self.white_rooks.pop_count() * PIECE_VALUES[Piece::WhiteRook as usize]
                    + self.white_queens.pop_count() * PIECE_VALUES[Piece::WhiteQueen as usize]
                    + self.white_king.pop_count() * PIECE_VALUES[Piece::WhiteKing as usize]
            }
            Color::Black => {
                self.black_pawns.pop_count() * PIECE_VALUES[Piece::BlackPawn as usize]
                    + self.black_knights.pop_count() * PIECE_VALUES[Piece::BlackKnight as usize]
                    + self.black_bishops.pop_count() * PIECE_VALUES[Piece::BlackBishop as usize]
                    + self.black_rooks.pop_count() * PIECE_VALUES[Piece::BlackRook as usize]
                    + self.black_queens.pop_count() * PIECE_VALUES[Piece::BlackQueen as usize]
                    + self.black_king.pop_count() * PIECE_VALUES[Piece::BlackKing as usize]
            }
        }
    }

    pub fn piece(&self, sq: &Square) -> Option<Piece> {
        let sq_bb: Bitboard = (*sq).into();
        if (sq_bb & self.white_pawns).0 != 0x0 {
            Some(Piece::WhitePawn)
        } else if (sq_bb & self.white_knights).0 != 0x0 {
            Some(Piece::WhiteKnight)
        } else if (sq_bb & self.white_bishops).0 != 0x0 {
            Some(Piece::WhiteBishop)
        } else if (sq_bb & self.white_rooks).0 != 0x0 {
            Some(Piece::WhiteRook)
        } else if (sq_bb & self.white_queens).0 != 0x0 {
            Some(Piece::WhiteQueen)
        } else if (sq_bb & self.white_king).0 != 0x0 {
            Some(Piece::WhiteKing)
        } else if (sq_bb & self.black_pawns).0 != 0x0 {
            Some(Piece::BlackPawn)
        } else if (sq_bb & self.black_knights).0 != 0x0 {
            Some(Piece::BlackKnight)
        } else if (sq_bb & self.black_bishops).0 != 0x0 {
            Some(Piece::BlackBishop)
        } else if (sq_bb & self.black_rooks).0 != 0x0 {
            Some(Piece::BlackRook)
        } else if (sq_bb & self.black_queens).0 != 0x0 {
            Some(Piece::BlackQueen)
        } else if (sq_bb & self.black_king).0 != 0x0 {
            Some(Piece::BlackKing)
        } else {
            None
        }
    }

    pub fn from_fen(fen: impl ToString) -> Result<Board, FENParsingError> {
        let mut board = Board::empty();
        let line_break = '/';
        let mut sq_counter: usize = 56; // start with A8 == 56

        for ch in fen.to_string().chars() {
            if ch == line_break {
                sq_counter -= 8 * 2;
                continue;
            }
            if let Some(d) = ch.to_digit(10) {
                sq_counter += d as usize;
            } else {
                let piece = Piece::try_from(ch)?;
                let sq = Square::try_from(sq_counter)?;
                let _ = board.add_piece(piece, sq);
                sq_counter += 1;
            }
        }
        Ok(board)
    }

    pub fn is_square_attacked(&self, sq: Square, color: Color) -> bool {
        let piece_array: &[Piece; 6] = match color {
            Color::White => &WHITE_PIECES,
            Color::Black => &BLACK_PIECES,
        };
        for piece in piece_array {
            let attack_dirs = DIRECTIONS[piece.attack_direction_idx()].to_vec();
            let attack_bb = self.bitboard(piece);

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
                if utils::recur_attack_sq_search(&self, attack_dirs, sq, 1, attack_bb) {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white_pawns: Rank::Rank2.into(),
            white_knights: Bitboard::from(vec![Square::B1, Square::G1]),
            white_bishops: Bitboard::from(vec![Square::C1, Square::F1]),
            white_rooks: Bitboard::from(vec![Square::A1, Square::H1]),
            white_queens: Square::D1.into(),
            white_king: Square::E1.into(),
            black_pawns: Rank::Rank7.into(),
            black_knights: Bitboard::from(vec![Square::B8, Square::G8]),
            black_bishops: Bitboard::from(vec![Square::C8, Square::F8]),
            black_rooks: Bitboard::from(vec![Square::A8, Square::H8]),
            black_queens: Square::D8.into(),
            black_king: Square::E8.into(),
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_br = "  +---+---+---+---+---+---+---+---+\n";
        f.write_str(line_br)?;
        for rank in RANKS.iter().rev() {
            f.write_str(format!("{} ", *rank as usize).as_str())?;
            for &file in FILES.iter() {
                let sq: Bitboard = Square::new(file, *rank).into();
                let s = if (self.white_bishops & sq).0 != 0x0 {
                    "| B "
                } else if (self.white_king & sq).0 != 0x0 {
                    "| K "
                } else if (self.white_knights & sq).0 != 0x0 {
                    "| N "
                } else if (self.white_queens & sq).0 != 0x0 {
                    "| Q "
                } else if (self.white_rooks & sq).0 != 0x0 {
                    "| R "
                } else if (self.white_pawns & sq).0 != 0x0 {
                    "| P "
                } else if (self.black_king & sq).0 != 0x0 {
                    "| k "
                } else if (self.black_knights & sq).0 != 0x0 {
                    "| n "
                } else if (self.black_bishops & sq).0 != 0x0 {
                    "| b "
                } else if (self.black_pawns & sq).0 != 0x0 {
                    "| p "
                } else if (self.black_queens & sq).0 != 0x0 {
                    "| q "
                } else if (self.black_rooks & sq).0 != 0x0 {
                    "| r "
                } else {
                    "|   "
                };
                f.write_str(s)?;
            }
            f.write_str("|\n")?;
            f.write_str(line_br)?;
        }
        f.write_str("    a   b   c   d   e   f   g   h  \n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;
    use crate::board::constants::SQUARES;

    fn rm_whitespace(s: impl ToString) -> String {
        let mut out = s.to_string();
        out.retain(|c| !c.is_whitespace());
        out
    }

    #[test]
    fn test_default_board() {
        let default_board_fmt = rm_whitespace(
            "
        +---+---+---+---+---+---+---+---+        
      8 | r | n | b | q | k | b | n | r |
        +---+---+---+---+---+---+---+---+
      7 | p | p | p | p | p | p | p | p |
        +---+---+---+---+---+---+---+---+
      6 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      5 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      4 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      3 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      2 | P | P | P | P | P | P | P | P |
        +---+---+---+---+---+---+---+---+
      1 | R | N | B | Q | K | B | N | R |
        +---+---+---+---+---+---+---+---+
          a   b   c   d   e   f   g   h      
      ",
        );
        assert_eq!(
            rm_whitespace(format!("{:?}", Board::default())),
            default_board_fmt
        );
    }

    #[test]
    fn test_empty_board() {
        let board = Board::empty();
        assert_eq!(board.bitboard_union(), Bitboard::empty());
    }

    #[test]
    fn test_bitboard_union() {
        let mut board = Board::empty();
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.bitboard_union(), Bitboard(0b10));
    }

    #[test]
    fn test_sq_taken() {
        let mut board = Board::empty();
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert!(board.sq_taken(Square::B1));
    }

    #[test]
    fn test_add_piece() {
        let mut board = Board::empty();
        assert_eq!(board.bitboard_union(), Bitboard::empty());
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.bitboard_union(), Bitboard(0b10));
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::empty();
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.bitboard_union(), Bitboard(0b10));
        let _ = board.remove_piece(Square::B1);
        assert_eq!(board.bitboard_union(), Bitboard::empty());
        assert!(board.remove_piece(Square::B1).is_err());
    }

    #[test]
    fn test_replace_piece() {
        let mut board = Board::default();
        assert_eq!(board.piece(&Square::A1), Some(Piece::WhiteRook));
        let _ = board.replace_piece(Square::A1, Piece::WhiteKnight);
        assert_eq!(board.piece(&Square::A1), Some(Piece::WhiteKnight));
        assert!(board.replace_piece(Square::D4, Piece::WhiteKing).is_err());
    }

    #[test]
    fn test_board_from_fen() {
        let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let parsed_board = Board::from_fen(start_pos).unwrap();
        assert_eq!(parsed_board, Board::default());
    }

    #[test]
    fn test_board_material() {
        let mut board_one = Board::empty();
        assert_eq!(board_one.material(Color::White), 0);
        assert_eq!(board_one.material(Color::Black), 0);
        let _ = board_one.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board_one.material(Color::White), 0);
        assert_eq!(board_one.material(Color::Black), 1000);
        let _ = board_one.add_piece(Piece::BlackRook, Square::C1);
        assert_eq!(board_one.material(Color::White), 0);
        assert_eq!(board_one.material(Color::Black), 1000 + 550);
        let _ = board_one.add_piece(Piece::WhitePawn, Square::F7);
        let _ = board_one.add_piece(Piece::WhitePawn, Square::F8);
        let _ = board_one.add_piece(Piece::WhitePawn, Square::G3);
        assert_eq!(board_one.material(Color::White), 3 * 100);
        assert_eq!(board_one.material(Color::Black), 1000 + 550);

        let mut board_two = Board::default();
        assert_eq!(
            board_two.material(Color::White),
            8 * 100 + 2 * 325 + 2 * 325 + 2 * 550 + 1000 + 50000
        );
        assert_eq!(
            board_two.material(Color::Black),
            8 * 100 + 2 * 325 + 2 * 325 + 2 * 550 + 1000 + 50000
        );
        let _ = board_two.remove_piece(Square::D8);
        assert_eq!(
            board_two.material(Color::White),
            8 * 100 + 2 * 325 + 2 * 325 + 2 * 550 + 1000 + 50000
        );
        assert_eq!(
            board_two.material(Color::Black),
            8 * 100 + 2 * 325 + 2 * 325 + 2 * 550 + 50000
        );
    }

    #[test]
    fn test_is_square_attacked() {
        let fen = "rn1qkb1r/pp2pppp/3p3n/2p2b2/4P3/2N2N2/PPPP1PPP/R1BQKB1R";
        let board = Board::from_fen(fen).unwrap();

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
            assert!(board.is_square_attacked(*sq, Color::White));
        }
        let squares_not_attacked_by_white = SQUARES
            .iter()
            .filter(|sq| !squares_attacked_by_white.contains(sq));
        for sq in squares_not_attacked_by_white {
            assert!(!board.is_square_attacked(*sq, Color::White));
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
            assert!(board.is_square_attacked(*sq, Color::Black));
        }
        let squares_not_attacked_by_black = SQUARES
            .iter()
            .filter(|sq| !squares_attacked_by_black.contains(sq));
        for sq in squares_not_attacked_by_black {
            assert!(!board.is_square_attacked(*sq, Color::Black));
        }
    }
}
