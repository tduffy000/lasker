use std::fmt::{self, Write};

use crate::board::{
    constants::{PIECES, SQUARES},
    types::{Piece, Square},
};

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
        f.write_char(from_file.into())?;
        f.write_char(from_rank.into())?;
        f.write_char(to_file.into())?;
        f.write_char(to_rank.into())?;
        if let Some(piece) = self.promoted() {
            let piece_c: char = piece.into();
            f.write_char(piece_c.to_ascii_lowercase())?
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
}
