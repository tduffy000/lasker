// https://github.com/official-stockfish/Stockfish/blob/master/src/types.h
#[rustfmt::skip]

use crate::board::Bitboard;
use std::convert::TryFrom;

const FILE_A: u64 = 0x0101010101010101;
const RANK_1: u64 = 0xFF;

pub trait EnumToArray<T, const N: usize> {
    fn array() -> [T; N];
}

pub enum Color {White, Black}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum File {
    A, B, C, D, E, F, G, H
}

pub const FILES: [File; 8] = [
    File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H
];

impl EnumToArray<File, 8> for File {
    fn array() -> [File; 8] {
        FILES
    }
}

impl Into<Bitboard> for File {
    fn into(self) -> Bitboard {
        Bitboard(FILE_A << self as usize)
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Rank {
    Rank1 = 1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
}

pub const RANKS: [Rank; 8] = [
    Rank::Rank1, Rank::Rank2, Rank::Rank3, Rank::Rank4, Rank::Rank5, Rank::Rank6, Rank::Rank7, Rank::Rank8,
];

impl EnumToArray<Rank, 8> for Rank {
    fn array() -> [Rank; 8] {
        RANKS
    }
}

impl Into<Bitboard> for Rank {
    fn into(self) -> Bitboard {
        Bitboard(RANK_1 << (8 * (self as usize - 1)))
    }
}

pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8, 
}

pub const SQUARES: [Square; 64] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8, 
];

impl EnumToArray<Square, 64> for Square {
    fn array() -> [Square; 64] {
        SQUARES
    }
}

#[derive(Debug)]
pub struct SquareIndexError {
    idx: usize,
    msg: String,
}

impl SquareIndexError {
    fn new(idx: usize, msg: impl ToString) -> Self {
        SquareIndexError { idx, msg: msg.to_string() }
    }
}

impl TryFrom<usize> for Square {
    type Error = SquareIndexError;
    
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if let Some(sq) = SQUARES.get(value) {
            Ok(*sq)
        } else {
            Err(SquareIndexError::new(value, "Square out of range!"))
        }
    }
}

impl Into<Bitboard> for Square {
    fn into(self) -> Bitboard {
        Bitboard(0x1 << self as usize)
    }
}

// can use Kernighan's algo here
fn set_bits(b: u64) -> Vec<usize> {
    let mut v = Vec::new();
    for sh in 0..64 {
        if b & (0x1 << sh) != 0x0 { v.push(sh) }
    }
    v
}

fn get_bit_index(b: u64) -> Option<usize> {
    if b == 0x0 { return None };
    if !(b & (b-1) == 0x0) { return None };
    for sh in 0..64 {
        if b & (0x1 << sh) != 0x0 { return Some(sh) }
    }
    None
}

fn get_square(bb: Bitboard) -> Option<Square> {
    if let Some(idx) = get_bit_index(bb.into()) {
        if let Some(sq) = SQUARES.get(idx) {
            return Some(*sq)
        }
    }
    None
}

impl Square {
    pub fn new(f: File, r: Rank) -> Self {
        let fbb: Bitboard = f.into();
        let rbb: Bitboard = r.into();
        get_square(fbb & rbb).unwrap()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_bits() {
        assert_eq!(set_bits(0x0), Vec::<usize>::new());
    }

    #[test]
    fn test_get_bit_index() {
        assert!(get_bit_index(0x0).is_none());
        assert_eq!(get_bit_index(0x08), Some(3));
    }

    #[test]
    fn test_get_square() {

    }

    #[test]
    fn test_try_from_usize_for_sq() {
        let sq_b1 = Square::try_from(1).unwrap();
        assert_eq!(sq_b1, Square::B1);

        let no_sq = Square::try_from(65);
        assert!(no_sq.is_err());   
    }

    #[test]
    fn test_file_to_bitboard() {
        let bb: Bitboard = File::B.into();
        let expected_bit_idx: Vec<usize> = vec![
            Square::B1, Square::B2, Square::B3, Square::B4,
            Square::B5, Square::B6, Square::B7, Square::B8
        ].iter().map(|sq| *sq as usize).collect();
        let set_bits = set_bits(bb.into());
        assert_eq!(expected_bit_idx, set_bits);
    }

    #[test]
    fn test_rank_to_bitboard() {
        let bb: Bitboard = Rank::Rank5.into();
        let expected_bit_idx: Vec<usize> = vec![
            Square::A5, Square::B5, Square::C5, Square::D5,
            Square::E5, Square::F5, Square::G5, Square::H5,
        ].iter().map(|sq| *sq as usize).collect();
        let set_bits = set_bits(bb.into());
        assert_eq!(expected_bit_idx, set_bits);
    }

}