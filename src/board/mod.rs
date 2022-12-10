use std::{convert::TryFrom, fmt::Debug};

pub mod key;

mod bitboard;
mod constants;
mod error;
mod types;
mod utils;

use bitboard::Bitboard;
use error::{FENParsingError, NoPieceOnSquareError, SquareTakenError};
use types::{CastlingRights, Color, EnumToArray, File, Piece, Rank, Square};

use self::constants::PIECE_VALUES;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardState {
    board: Board,
    side_to_move: Color,
    en_passant: Option<Square>,
    fifty_move_counter: usize,
    ply: usize,
    history_ply: usize,
    position_key: u64,
    castling_permissions: CastlingRights, // bits = [ wK, wQ, bK, bQ ]
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Board {
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
    fn empty() -> Self {
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

    fn taken(&self) -> Bitboard {
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

    fn add_piece(&mut self, piece: Piece, sq: Square) -> Result<(), SquareTakenError> {
        let sq_bb: Bitboard = sq.into();
        if (self.taken() & sq_bb).0 != 0x0 {
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

    // do we really need to specify the piece here?
    fn remove_piece(&mut self, piece: Piece, sq: Square) -> Result<(), NoPieceOnSquareError> {
        let sq_bb: Bitboard = sq.into();
        if (self.taken() & sq_bb).0 == 0x0 {
            Err(NoPieceOnSquareError::new(sq))
        } else {
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
            Ok(())
        }
    }

    fn pieces(&self, color: Color) -> Bitboard {
        match color {
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
                    | self.black_knights
                    | self.black_bishops
                    | self.black_rooks
                    | self.black_queens
                    | self.black_king
            }
        }
    }

    fn material(&self, color: Color) -> u32 {
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

    fn piece(&self, sq: &Square) -> Option<Piece> {
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

    fn from_fen(fen: impl ToString) -> Result<Board, FENParsingError> {
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

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_br = "  +---+---+---+---+---+---+---+---+\n";
        f.write_str(line_br)?;
        for rank in Rank::array().iter().rev() {
            f.write_str(format!("{} ", *rank as usize).as_str())?;
            for file in File::array().iter() {
                let sq: Bitboard = Square::new(*file, *rank).into();
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

    use super::*;

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
        assert_eq!(board.taken(), Bitboard::empty());
    }

    #[test]
    fn test_taken() {
        let mut board = Board::empty();
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.taken(), Bitboard(0b10));
    }

    #[test]
    fn test_add_piece() {
        let mut board = Board::empty();
        assert_eq!(board.taken(), Bitboard::empty());
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.taken(), Bitboard(0b10));
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::empty();
        let _ = board.add_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.taken(), Bitboard(0b10));
        let _ = board.remove_piece(Piece::BlackQueen, Square::B1);
        assert_eq!(board.taken(), Bitboard::empty());
        assert!(board.remove_piece(Piece::BlackQueen, Square::B1).is_err());
    }

    #[test]
    fn test_board_from_fen() {
        let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let parsed_board = Board::from_fen(start_pos).unwrap();
        assert_eq!(parsed_board, Board::default());
    }

    #[test]
    fn test_board_state_from_fen() {
        let start_state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed_state = BoardState::from_fen(start_state).unwrap();
        assert_eq!(parsed_state, BoardState::default());
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
        let _ = board_two.remove_piece(Piece::BlackQueen, Square::D8);
        assert_eq!(
            board_two.material(Color::White),
            8 * 100 + 2 * 325 + 2 * 325 + 2 * 550 + 1000 + 50000
        );
        assert_eq!(
            board_two.material(Color::Black),
            8 * 100 + 2 * 325 + 2 * 325 + 2 * 550 + 50000
        );
    }
}
