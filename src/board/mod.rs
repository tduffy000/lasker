use std::fmt::Debug;

mod bitboard;
mod error;
mod types;
mod utils;

use crate::board::types::EnumToArray;
use bitboard::Bitboard;
use types::{Color, File, Rank, Square};

pub struct BoardState {
    board: Board,
    side_to_move: Color,
    en_passant: Option<Square>,
    fifth_move_counter: usize,
    ply: usize,
    history_ply: usize,
    position_key: u64,        // unique identifier of the position
    castling_permissions: u8, // bits = [ wK, wQ, bK, bQ ]
}

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
        let line_br = "+---+---+---+---+---+---+---+---+\n";
        f.write_str(line_br)?;
        for rank in Rank::array().iter().rev() {
            f.write_str(format!("{} ", *rank as usize).as_str())?;
            for file in File::array().iter() {
                let sq: Bitboard = Square::new(*file, *rank).into();
                let s = if (self.white_bishops & sq).0 != 0x0 {
                    " | b "
                } else if (self.white_king & sq).0 != 0x0 {
                    " | k "
                } else if (self.white_knights & sq).0 != 0x0 {
                    " | n "
                } else if (self.white_queens & sq).0 != 0x0 {
                    " | q "
                } else if (self.white_rooks & sq).0 != 0x0 {
                    " | r "
                } else if (self.white_pawns & sq).0 != 0x0 {
                    " | p "
                } else if (self.black_king & sq).0 != 0x0 {
                    " | K "
                } else if (self.black_knights & sq).0 != 0x0 {
                    " | N "
                } else if (self.black_bishops & sq).0 != 0x0 {
                    " | B "
                } else if (self.black_pawns & sq).0 != 0x0 {
                    " | P "
                } else if (self.black_queens & sq).0 != 0x0 {
                    " | Q "
                } else if (self.black_rooks & sq).0 != 0x0 {
                    " | R "
                } else {
                    "|  "
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
      8 | R | N | B | Q | K | B | N | R |
        +---+---+---+---+---+---+---+---+
      7 | P | P | P | P | P | P | P | P |
        +---+---+---+---+---+---+---+---+
      6 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      5 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      4 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      3 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      2 | p | p | p | p | p | p | p | p |
        +---+---+---+---+---+---+---+---+
      1 | r | n | b | q | k | b | n | r |
        +---+---+---+---+---+---+---+---+
          a   b   c   d   e   f   g   h      
      ",
        );
        assert_eq!(
            rm_whitespace(format!("{:?}", Board::default())),
            default_board_fmt
        );
    }
}
