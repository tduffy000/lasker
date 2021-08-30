mod bitboard;
mod types;

use bitboard::Bitboard;
use types::{Color, Square, Rank};

pub struct BoardState {
    board: Board,
    side_to_move: Color,
    en_passant: Option<Square>,
    fifth_move_counter: usize,
    ply: usize,
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
    fn pieces(self, color: Color) -> Bitboard {
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


// impl Debug for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let line_br = "+---+---+---+---+---+---+---+---+\n";
//         f.write_str(line_br)?;
//         for rank in Rank::array().iter().rev() {
//             // print pieces
//             for file in File::array().iter().rev() {
//                 // print pieces
//             }
//             f.write_str("|\n")?;
//             f.write_str(line_br)?;
//         }
//         f.write_str("    a   b   c   d   e   f   g   h  \n")?;
//         Ok(())
//     }
// }


#[cfg(test)]
mod tests {

    #[test]
    fn test_default_board() {

    }

}