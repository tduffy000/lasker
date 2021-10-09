use std::convert::TryFrom;

use crate::board::{error::SquareIndexError, Bitboard};

// using Little-Endian Rank File Mapping
// @see https://www.chessprogramming.org/Square_Mapping_Considerations
const FILE_A: u64 = 0x0101010101010101;
const RANK_1: u64 = 0xFF;
const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
const H1_A1_DIAGONAL: u64 = 0x0102040810204080;

pub trait EnumToArray<T, const N: usize> {
    fn array() -> [T; N];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub const FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum Rank {
    Rank1 = 1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
}

pub const RANKS: [Rank; 8] = [
    Rank::Rank1,
    Rank::Rank2,
    Rank::Rank3,
    Rank::Rank4,
    Rank::Rank5,
    Rank::Rank6,
    Rank::Rank7,
    Rank::Rank8,
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

impl Into<char> for Piece {
    fn into(self) -> char {
        match self {
            Self::WhitePawn => 'p',
            Self::WhiteKnight => 'n',
            Self::WhiteBishop => 'b',
            Self::WhiteRook => 'r',
            Self::WhiteQueen => 'q',
            Self::WhiteKing => 'k',
            Self::BlackPawn => 'P',
            Self::BlackKnight => 'N',
            Self::BlackBishop => 'B',
            Self::BlackRook => 'R',
            Self::BlackQueen => 'Q',
            Self::BlackKing => 'K',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

pub const SQUARES: [Square; 64] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,
    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8,
];

impl EnumToArray<Square, 64> for Square {
    fn array() -> [Square; 64] {
        SQUARES
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

impl Square {
    pub fn new(f: File, r: Rank) -> Self {
        let fbb: Bitboard = f.into();
        let rbb: Bitboard = r.into();
        let sq: Vec<Square> = (fbb & rbb).into();
        assert_eq!(sq.len(), 1);
        sq[0]
    }

    pub fn rank(&self) -> Rank {
        let pos = *self as usize;
        let rank_pos = pos >> 3;
        *Rank::array().get(rank_pos).unwrap()
    }

    pub fn file(&self) -> File {
        let pos = *self as usize;
        let file_pos = pos & 7;
        *File::array().get(file_pos).unwrap()
    }
}

pub enum Direction {
    North = 1,
    NorthEast = 9,
    SouthEast = 7,
    South = -1,
    SouthWest = -9,
    West = -8,
    NorthWest = -7,
}

// four bits to represent castling
// so 2 ^ {0..3}
pub enum CastlingRight {
    WhiteKing = 1,
    WhiteQueen = 2,
    BlackKing = 4,
    BlackQueen = 8,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::utils::set_bits;

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
            Square::B1,
            Square::B2,
            Square::B3,
            Square::B4,
            Square::B5,
            Square::B6,
            Square::B7,
            Square::B8,
        ]
        .iter()
        .map(|sq| *sq as usize)
        .collect();
        let set_bits = set_bits(bb.into());
        assert_eq!(expected_bit_idx, set_bits);
    }

    #[test]
    fn test_rank_to_bitboard() {
        let bb: Bitboard = Rank::Rank5.into();
        let expected_bit_idx: Vec<usize> = vec![
            Square::A5,
            Square::B5,
            Square::C5,
            Square::D5,
            Square::E5,
            Square::F5,
            Square::G5,
            Square::H5,
        ]
        .iter()
        .map(|sq| *sq as usize)
        .collect();
        let set_bits = set_bits(bb.into());
        assert_eq!(expected_bit_idx, set_bits);
    }

    #[test]
    fn test_square_new() {
        let f = File::C;
        let r = Rank::Rank3;
        let sq = Square::new(f, r);
        assert_eq!(sq, Square::C3);
    }

    #[test]
    fn test_square_file() {
        let sq = Square::D4;
        assert_eq!(sq.file(), File::D);
    }

    #[test]
    fn test_square_rank() {
        let sq = Square::F7;
        assert_eq!(sq.rank(), Rank::Rank7);
    }

    #[test]
    fn test_square_into_bitboard() {
        let sq = Square::B1;
        let sq_bb: Bitboard = sq.into();
        assert_eq!(sq_bb, Bitboard(0b10));
    }

    #[test]
    fn test_square_from_usize() {
        let valid_idx = 10;
        let res_valid = Square::try_from(valid_idx);
        assert!(res_valid.is_ok());
        if let Ok(sq) = res_valid {
            assert_eq!(sq, Square::C2);
        }

        let invalid_idx = 100;
        let res_invalid = Square::try_from(invalid_idx);
        assert!(res_invalid.is_err());
    }
}
