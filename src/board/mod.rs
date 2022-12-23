use std::{convert::TryFrom, fmt::Debug};

mod bitboard;
pub(in crate::board) mod constants;
mod error;
pub mod key;
pub mod r#move;
pub mod types;
mod utils;

use bitboard::Bitboard;
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
                        self.board.recur_move_search(color, dirs, &mut moves, sq, 1);
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    fn sq_taken(&self, sq: Square) -> bool {
        (self.bitboard_union() & sq.into()).0 != 0x0
    }

    fn sq_taken_by_color(&self, sq: Square, color: Color) -> bool {
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

    // TODO (tcd 12/21/22): only promotes to Queens rn
    // TODO (tcd 12/21/22): optimization opportunity: return a ring buffer or something
    // already alloc'ed as opposed to a Vec<>; also this could be multi-threaded?

    fn recur_move_search(
        &self,
        color: Color,
        dirs: &Vec<Direction>,
        moves: &mut Vec<Move>,
        sq: Square,
        depth: i8,
    ) -> () {
        let mut to_search = vec![];
        for dir in dirs {
            let mailbox_no = sq + (*dir as i8 * depth);
            if mailbox_no >= 0 {
                let other_sq = Square::from_mailbox_no(mailbox_no);
                if !self.sq_taken_by_color(other_sq, color) {
                    let captured = self.piece(&other_sq);
                    moves.push(Move::new(sq, other_sq, captured, None, false, false, false));
                    if captured.is_none() {
                        to_search.push(*dir)
                    }
                }
            }
        }
        if !to_search.is_empty() {
            return self.recur_move_search(color, &to_search, moves, sq, depth + 1);
        }
    }

    fn recur_attack_sq_search(
        &self,
        dirs: Vec<Direction>,
        sq: Square,
        depth: i8,
        attack_bb: Bitboard,
    ) -> bool {
        let mut to_search = vec![];
        for dir in dirs {
            let mailbox_no = sq + (dir as i8 * depth);
            if mailbox_no >= 0 {
                let other_sq = Square::from_mailbox_no(mailbox_no);
                let other_sq_bb: Bitboard = other_sq.into();
                if (other_sq_bb & attack_bb).0 != 0x0 {
                    return true;
                } else if !self.sq_taken(other_sq) {
                    to_search.push(dir);
                }
            }
        }
        if !to_search.is_empty() {
            return self.recur_attack_sq_search(to_search, sq, depth + 1, attack_bb);
        } else {
            return false;
        }
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
                if self.recur_attack_sq_search(attack_dirs, sq, 1, attack_bb) {
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

impl Debug for Board {
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
    use constants::SQUARES;

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
    fn test_board_pieces() {
        todo!()
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
