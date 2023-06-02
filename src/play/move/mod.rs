use std::fmt::{self, Write};

use crate::play::{
    constants::{PIECES, SQUARES},
    types::{Piece, Square},
};

use super::{
    types::{CastlingRight, CastlingRights, Color, Direction, File},
    GameState, error::MoveError,
};

///
///
pub fn make_move(mv: Move, state: &mut GameState) -> Result<(), MoveError> {

    // update 50-move counter before the pieces are moved on the board
    if (mv.captured().is_some())
        | (state.position.board.piece(&mv.from_sq()) == Some(Piece::WhitePawn))
        | (state.position.board.piece(&mv.from_sq()) == Some(Piece::BlackPawn))
    {
        state.fifty_move_counter = 0;
    } else {
        state.fifty_move_counter += 1;
    }

    if mv.captured().is_some() {
        state.position.board.remove_piece(mv.to_sq())?;
    }
    state.position.board.move_piece(mv.from_sq(), mv.to_sq())?;
    if mv.castle() {
        if mv.from_sq() == Square::E1 {
            if mv.to_sq() == Square::G1 {
                state
                    .position
                    .board
                    .move_piece(Square::H1, Square::F1)?
            } else if mv.to_sq() == Square::C1 {
                state
                    .position
                    .board
                    .move_piece(Square::A1, Square::D1)?
            }
            state.position.castling_permissions.unset_white_bits();
        } else if mv.from_sq() == Square::E8 {
            if mv.to_sq() == Square::G8 {
                state
                    .position
                    .board
                    .move_piece(Square::H8, Square::F8)?
            } else if mv.to_sq() == Square::C8 {
                state
                    .position
                    .board
                    .move_piece(Square::A8, Square::D8)?
            }

            state.position.castling_permissions.unset_black_bits();
        }
    }
    if (mv.from_sq() == Square::A1)
        & (state.position.board.piece(&Square::A1) == Some(Piece::WhiteRook))
    {
        state.position.castling_permissions.0 &=
            CastlingRights::all().0 - CastlingRight::WhiteQueen as u8;
    } else if (mv.from_sq() == Square::H1)
        & (state.position.board.piece(&Square::H1) == Some(Piece::WhiteRook))
    {
        state.position.castling_permissions.0 &=
            CastlingRights::all().0 - CastlingRight::WhiteKing as u8;
    } else if (mv.from_sq() == Square::A8)
        & (state.position.board.piece(&Square::A8) == Some(Piece::BlackRook))
    {
        state.position.castling_permissions.0 &=
            CastlingRights::all().0 - CastlingRight::BlackQueen as u8;
    } else if (mv.from_sq() == Square::H8)
        & (state.position.board.piece(&Square::H8) == Some(Piece::BlackRook))
    {
        state.position.castling_permissions.0 &=
            CastlingRights::all().0 - CastlingRight::BlackKing as u8;
    };

    if mv.pawn_start() {
        let dir = match state.position.side_to_move {
            Color::White => Direction::South,
            Color::Black => Direction::North,
        };
        let sq = Square::from_mailbox_no(mv.to_sq() + dir as i8);
        state.position.en_passant = Some(sq);
    } else {
        state.position.en_passant = None;
    }
    if mv.en_passant() {
        let dir = match state.position.side_to_move {
            Color::White => Direction::South, // white moving, capture black pawn on sq south of en passant sq
            Color::Black => Direction::North, // black moving, capture white pawn on sq north of en passant sq
        };
        let capture_sq = Square::from_mailbox_no(state.position.en_passant.unwrap() + dir as i8);
        let _ = state.position.board.remove_piece(capture_sq)?;
    }

    // TODO: pos key

    if let Some(piece) = mv.promoted() {
        state.position.board.remove_piece(mv.to_sq())?;
        state.position.board.add_piece(piece, mv.to_sq())?;
    }

    state.ply += 1;
    state.position.flip_side();
    state.fifty_move_country_hist.push(state.fifty_move_counter);
    state.position.castling_perms_history.push(state.position.castling_permissions);
    Ok(())
}

///
///
pub fn unmake_move(mv: Move, state: &mut GameState) -> Result<(), MoveError> {
    state.position.board.move_piece(mv.to_sq(), mv.from_sq())?;
    
    // is en passant a capture? if so this is wrong for en passant captures
    if let Some(piece) = mv.captured() {
        state.position.board.add_piece(piece, mv.to_sq())?;
    }

    // TODO: idk about the first castling condition

    if mv.pawn_start() {

    }

    if mv.en_passant() {

    }

    if let Some(piece) = mv.promoted() {
        state.position.board.remove_piece(mv.to_sq())?;
        let pawn = match piece.color() {
            Color::White => Piece::WhitePawn,
            Color::Black => Piece::BlackPawn,
        };
        state.position.board.add_piece(pawn, mv.from_sq())?;
    }

    state.ply -= 1;
    state.position.flip_side();
    state.fifty_move_counter = state.fifty_move_country_hist.pop().unwrap();
    state.position.castling_permissions = state.position.castling_perms_history.pop().unwrap();
    Ok(())
}

const MOVE_LIST_SIZE: usize = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    repr: u32,
    pub score: i8,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_file = &self.from_sq().file();
        let from_rank = &self.from_sq().rank();
        let to_file = &self.to_sq().file();
        let to_rank = &self.to_sq().rank();

        if self.castle() & (*to_file == File::C) {
            f.write_str("0-0-0");
        } else if self.castle() & (*to_file == File::G) {
            f.write_str("0-0");
        } else {
            f.write_char(from_file.into())?;
            f.write_char(from_rank.into())?;
            f.write_char(to_file.into())?;
            f.write_char(to_rank.into())?;
            if let Some(piece) = self.promoted() {
                let piece_c: char = piece.into();
                f.write_char(piece_c.to_ascii_lowercase())?
            }
        }
        Ok(())
    }
}

impl Move {
    pub fn empty() -> Move {
        Move {
            repr: 0x0,
            score: 0,
        }
    }

    pub fn new(
        from: Square,
        to: Square,
        captured: Option<Piece>,
        promoted: Option<Piece>,
        en_passant: bool,
        pawn_start: bool,
        castle: bool,
    ) -> Move {
        let from_sq_bits = from as u32;
        let to_sq_bits = (to as u32) << 6;
        let captured_piece_bits = match captured {
            Some(piece) => piece as u32 + 1,
            None => 0x0,
        } << 12;
        let promoted_piece_bits = match promoted {
            Some(piece) => piece as u32 + 1,
            None => 0x0,
        } << 16;
        let en_passant_bit = if en_passant { 0b1 << 20 } else { 0b0 << 20 };
        let pawn_start_bit = if pawn_start { 0b1 << 21 } else { 0b0 << 21 };
        let castle_bit = if castle { 0b1 << 22 } else { 0b0 << 22 };

        Move {
            repr: from_sq_bits
                | to_sq_bits
                | captured_piece_bits
                | promoted_piece_bits
                | en_passant_bit
                | pawn_start_bit
                | castle_bit,
            score: 0,
        }
    }

    pub fn from_sq(&self) -> Square {
        let idx = self.repr & 0x3F;
        SQUARES[idx as usize]
    }

    pub fn to_sq(&self) -> Square {
        let idx = (self.repr >> 6) & 0x3F;
        SQUARES[idx as usize]
    }

    pub fn captured(&self) -> Option<Piece> {
        let idx = (self.repr >> 12) & 0xF;
        if idx == 0 {
            None
        } else {
            Some(PIECES[idx as usize - 1])
        }
    }

    pub fn promoted(&self) -> Option<Piece> {
        let idx = (self.repr >> 16) & 0xF;
        if idx == 0 {
            None
        } else {
            Some(PIECES[idx as usize - 1])
        }
    }

    pub fn en_passant(&self) -> bool {
        ((self.repr >> 20) & (0b1 as u32)) == 0b1
    }

    pub fn pawn_start(&self) -> bool {
        ((self.repr >> 21) & (0b1 as u32)) == 0b1
    }

    pub fn castle(&self) -> bool {
        ((self.repr >> 22) & (0b1 as u32)) == 0b1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MoveList {
    inner: [Move; MOVE_LIST_SIZE],
    count: u8,
    pos: u8,
}

impl Iterator for MoveList {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.count {
            None
        } else {
            let idx = self.pos as usize;
            self.pos += 1;
            Some(self.inner[idx])
        }
    }
}

impl MoveList {
    pub fn new(v: Vec<Move>) -> MoveList {
        let mut l = MoveList::empty();
        for el in v {
            l.push(el)
        }
        l
    }

    pub fn empty() -> MoveList {
        MoveList {
            inner: [Move::empty(); MOVE_LIST_SIZE],
            count: 0,
            pos: 0,
        }
    }

    pub fn sorted(&self) -> MoveList {
        let mut v = self.inner.to_vec();
        let _ = v.sort();
        MoveList::new(v)
    }

    pub fn push(&mut self, mv: Move) -> () {
        if self.count == MOVE_LIST_SIZE as u8 {
            panic!("MoveList full!");
        } else {
            self.inner[self.count as usize] = mv;
            self.count += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_display() {
        let mv = Move::new(Square::C3, Square::C4, None, None, false, false, false);
        assert_eq!(format!("{}", mv), "c3c4");

        let mv = Move::new(
            Square::H7,
            Square::H8,
            None,
            Some(Piece::WhiteQueen),
            false,
            false,
            false,
        );
        assert_eq!(format!("{}", mv), "h7h8q");
    }

    #[test]
    fn test_move_new() {
        let mv = Move::new(Square::C3, Square::C4, None, None, false, false, false);
        assert_eq!(mv.from_sq(), Square::C3);
        assert_eq!(mv.to_sq(), Square::C4);
        assert!(mv.captured().is_none());
        assert!(mv.promoted().is_none());
        assert!(!mv.en_passant());
        assert!(!mv.pawn_start());
        assert!(!mv.castle());

        let mv = Move::new(
            Square::H8,
            Square::H7,
            Some(Piece::BlackBishop),
            None,
            false,
            false,
            false,
        );
        assert_eq!(mv.from_sq(), Square::H8);
        assert_eq!(mv.to_sq(), Square::H7);
        assert_eq!(mv.captured(), Some(Piece::BlackBishop));
        assert!(mv.promoted().is_none());
        assert!(!mv.en_passant());
        assert!(!mv.pawn_start());
        assert!(!mv.castle());

        let mv = Move::new(
            Square::D5,
            Square::H1,
            Some(Piece::WhiteKnight),
            Some(Piece::BlackQueen),
            false,
            false,
            false,
        );
        assert_eq!(mv.from_sq(), Square::D5);
        assert_eq!(mv.to_sq(), Square::H1);
        assert_eq!(mv.captured(), Some(Piece::WhiteKnight));
        assert_eq!(mv.promoted(), Some(Piece::BlackQueen));
        assert!(!mv.en_passant());
        assert!(!mv.pawn_start());
        assert!(!mv.castle());

        let mv = Move::new(Square::A2, Square::A4, None, None, true, true, false);
        assert_eq!(mv.from_sq(), Square::A2);
        assert_eq!(mv.to_sq(), Square::A4);
        assert!(mv.captured().is_none());
        assert!(mv.promoted().is_none());
        assert!(mv.en_passant());
        assert!(mv.pawn_start());
        assert!(!mv.castle());

        let mv = Move::new(Square::E1, Square::B1, None, None, false, false, true);
        assert_eq!(mv.from_sq(), Square::E1);
        assert_eq!(mv.to_sq(), Square::B1);
        assert!(mv.captured().is_none());
        assert!(mv.promoted().is_none());
        assert!(!mv.en_passant());
        assert!(!mv.pawn_start());
        assert!(mv.castle());
    }

    #[test]
    fn test_make_move_simple() {
        let mut state = GameState::default();
        let mv = Move::new(
            Square::A2,
            Square::A3,
            None,
            None,
            false,
            true,
            false,
        );
        assert_eq!(state.position.board.piece(&Square::A3), None);
        assert_eq!(state.fifty_move_counter, 0);
        assert_eq!(state.ply, 0);
        make_move(mv, &mut state);
        assert_eq!(state.position.board.piece(&Square::A3), Some(Piece::WhitePawn));
        assert_eq!(state.fifty_move_counter, 0);
        assert_eq!(state.ply, 1);
    }

    #[test]
    fn test_unmake_move_simple() {
        let mut state = GameState::default();
        let mv = Move::new(
            Square::A2,
            Square::A3,
            None,
            None,
            false,
            true,
            false,
        );
        assert_eq!(state.position.board.piece(&Square::A3), None);
        assert_eq!(state.fifty_move_counter, 0);
        assert_eq!(state.ply, 0);
        make_move(mv, &mut state);
        assert_eq!(state.position.board.piece(&Square::A3), Some(Piece::WhitePawn));
        assert_eq!(state.fifty_move_counter, 0);
        assert_eq!(state.ply, 1);
        unmake_move(mv, &mut state);
        assert_eq!(state.position.board.piece(&Square::A3), None);
        assert_eq!(state.fifty_move_counter, 0);
        assert_eq!(state.ply, 0);
    }

    #[test]
    fn test_make_move_castle() {}

    #[test]
    fn test_unmake_move_castle() {}

    #[test]
    fn test_make_move_castling_rights() {}

    #[test]
    fn test_unmake_move_castling_rights() {}

    #[test]
    fn test_make_move_en_passant() {}

    #[test]
    fn test_unmake_move_en_passant() {}

    #[test]
    fn test_make_move_capture() {}

    #[test]
    fn test_unmake_move_capture() {}

    #[test]
    fn test_make_move_promotion() {}

    #[test]
    fn test_unmake_move_promotion() {}

    #[test]
    fn test_make_move_pawn_start() {}

    #[test]
    fn test_unmake_move_pawn_start() {}

    #[test]
    fn test_move_list_push() {
        let mut l = MoveList::empty();
        assert_eq!(l.count, 0);
        l.push(Move::empty());
        l.push(Move::empty());
        assert_eq!(l.count, 2);
    }

}
