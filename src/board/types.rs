use std::convert::TryFrom;

use crate::board::{
    error::{FENParsingError, InvalidCharError, SquareIndexError},
    Bitboard,
};

use crate::board::constants::{
    FILES, FILE_A, IS_MAJOR_PIECE, IS_MINOR_PIECE, RANKS, RANK_1, SQUARES,
};

const FEN_BLANK: &str = "-";

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

impl Into<Bitboard> for File {
    fn into(self) -> Bitboard {
        Bitboard(FILE_A << self as usize)
    }
}

impl TryFrom<char> for File {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        match alpha.iter().position(|&el| el == value) {
            Some(idx) => Ok(FILES[idx]),
            None => Err(InvalidCharError::new(value)),
        }
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

impl Into<Bitboard> for Rank {
    fn into(self) -> Bitboard {
        Bitboard(RANK_1 << (8 * (self as usize - 1)))
    }
}

impl TryFrom<char> for Rank {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(mut digit) = value.to_digit(10) {
            digit -= 1;
            if digit > 8 {
                Err(InvalidCharError::new(value))
            } else {
                Ok(RANKS[digit as usize])
            }
        } else {
            Err(InvalidCharError::new(value))
        }
    }
}

#[repr(usize)]
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

impl Piece {
    fn is_minor(self) -> bool {
        IS_MINOR_PIECE[self as usize]
    }

    fn is_major(self) -> bool {
        IS_MAJOR_PIECE[self as usize]
    }
}

impl Into<char> for Piece {
    fn into(self) -> char {
        match self {
            Self::WhitePawn => 'P',
            Self::WhiteKnight => 'N',
            Self::WhiteBishop => 'B',
            Self::WhiteRook => 'R',
            Self::WhiteQueen => 'Q',
            Self::WhiteKing => 'K',
            Self::BlackPawn => 'p',
            Self::BlackKnight => 'n',
            Self::BlackBishop => 'b',
            Self::BlackRook => 'r',
            Self::BlackQueen => 'q',
            Self::BlackKing => 'k',
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece::WhitePawn),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'R' => Ok(Piece::WhiteRook),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            'p' => Ok(Piece::BlackPawn),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'r' => Ok(Piece::BlackRook),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            c => Err(InvalidCharError::new(c)),
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
        *RANKS.get(rank_pos).unwrap()
    }

    pub fn file(&self) -> File {
        let pos = *self as usize;
        let file_pos = pos & 7;
        *FILES.get(file_pos).unwrap()
    }

    pub fn from_fen(fen: impl ToString) -> Result<Option<Square>, InvalidCharError> {
        if fen.to_string() == FEN_BLANK {
            return Ok(None);
        }
        let chars: Vec<char> = fen.to_string().chars().collect();
        if chars.len() != 2 {
            return Err(InvalidCharError::new('x'));
        }

        let f = File::try_from(chars[0])?;
        let r = Rank::try_from(chars[1])?;

        Ok(Some(Square::new(f, r)))
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
#[repr(u8)]
pub enum CastlingRight {
    WhiteKing = 1,
    WhiteQueen = 2,
    BlackKing = 4,
    BlackQueen = 8,
}

impl TryFrom<char> for CastlingRight {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'K' => Ok(CastlingRight::WhiteKing),
            'Q' => Ok(CastlingRight::WhiteQueen),
            'k' => Ok(CastlingRight::BlackKing),
            'q' => Ok(CastlingRight::BlackQueen),
            ch => Err(InvalidCharError::new(ch)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastlingRights(pub u8);

// TODO: from_fen should be a trait
impl CastlingRights {
    pub fn from_fen(fen: impl ToString) -> Result<Self, FENParsingError> {
        let mut rights = CastlingRights(0x0);
        if fen.to_string() == FEN_BLANK {
            return Ok(rights);
        }

        for ch in fen.to_string().chars() {
            rights.0 |= CastlingRight::try_from(ch)? as u8;
        }

        Ok(rights)
    }
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
    fn test_try_from_fen_sq() {
        assert!(Square::from_fen("xx").is_err());
        assert!(Square::from_fen("-").unwrap().is_none());
        assert_eq!(Square::from_fen("a8").unwrap().unwrap(), Square::A8);
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
    fn test_try_from_char_for_file() {
        assert!(File::try_from('x').is_err());
        assert_eq!(File::try_from('a').unwrap(), File::A);
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
    fn test_try_from_char_for_rank() {
        assert!(Rank::try_from('x').is_err());
        assert_eq!(Rank::try_from('1').unwrap(), Rank::Rank1);
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

    #[test]
    fn test_castling_rights_from_fen() {
        let white_queenside = "Q";
        let all = "KQkq";

        let empty_rights = CastlingRights::from_fen("").unwrap();
        let wq = CastlingRights::from_fen(white_queenside).unwrap();
        let all_rights = CastlingRights::from_fen(all).unwrap();

        assert_eq!(empty_rights.0, 0b0);
        assert_eq!(wq.0, CastlingRight::WhiteQueen as u8);
        assert_eq!(all_rights.0, 0b1111);

        assert!(CastlingRights::from_fen("X").is_err());
    }
}
