mod bitboard;
mod types;

use bitboard::Bitboard;
use types::{Color, Square};

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

// impl Default for Board {
//     fn default() -> Self {

//     }
// }
