use std::{
    convert::TryFrom,
    fmt::{self, Write},
    ops::{Add, Range},
};

use crate::play::{
    board::bitboard::Bitboard,
    constants::{
        FILES, FILE_A, IS_MAJOR_PIECE, IS_MINOR_PIECE, MAILBOX, MAILBOX_IDX, RANKS, RANK_1, SQUARES,
    },
    error::{FENParsingError, InvalidCharError},
};

use super::error::SquareIndexError;

const FEN_BLANK: &str = "-";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
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

// TODO (tcd 12/17/22): impl IntoIterator for Rank + File to get back Squares
impl Into<char> for &File {
    fn into(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.into())?;
        Ok(())
    }
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
#[repr(u8)]
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

impl Into<char> for &Rank {
    fn into(self) -> char {
        match self {
            Rank::Rank1 => '1',
            Rank::Rank2 => '2',
            Rank::Rank3 => '3',
            Rank::Rank4 => '4',
            Rank::Rank5 => '5',
            Rank::Rank6 => '6',
            Rank::Rank7 => '7',
            Rank::Rank8 => '8',
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.into());
        Ok(())
    }
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

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn color(&self) -> Color {
        match self {
            Piece::WhitePawn
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteRook
            | Piece::WhiteQueen
            | Piece::WhiteKing => Color::White,
            Piece::BlackPawn
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackRook
            | Piece::BlackQueen
            | Piece::BlackKing => Color::Black,
        }
    }

    pub fn opposing_color(&self) -> Color {
        match self.color() {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn attack_direction_idx(&self) -> Range<usize> {
        match self {
            Piece::WhitePawn => 4..6,
            Piece::BlackPawn => 6..8,
            Piece::WhiteKnight | Piece::BlackKnight => 8..16,
            Piece::WhiteBishop | Piece::BlackBishop => 4..8,
            Piece::WhiteRook | Piece::BlackRook => 0..4,
            Piece::WhiteQueen | Piece::BlackQueen | Piece::WhiteKing | Piece::BlackKing => 0..8,
        }
    }

    pub fn move_direction_idx(&self) -> Range<usize> {
        match self {
            Piece::WhitePawn => 6..8,
            Piece::BlackPawn => 4..6,
            Piece::WhiteKnight | Piece::BlackKnight => 8..16,
            Piece::WhiteBishop | Piece::BlackBishop => 4..8,
            Piece::WhiteRook | Piece::BlackRook => 0..4,
            Piece::WhiteQueen | Piece::BlackQueen | Piece::WhiteKing | Piece::BlackKing => 0..8,
        }
    }

    pub fn can_slide(&self) -> bool {
        match self {
            Piece::WhitePawn
            | Piece::BlackPawn
            | Piece::WhiteKnight
            | Piece::BlackKnight
            | Piece::WhiteKing
            | Piece::BlackKing => false,
            _ => true,
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Square {
    A1 = 0,
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

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = &self.rank();
        let file = &self.file();
        f.write_char(file.into())?;
        f.write_char(rank.into())?;
        Ok(())
    }
}

impl TryFrom<usize> for Square {
    type Error = SquareIndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if let Some(sq) = SQUARES.get(value) {
            Ok(*sq)
        } else {
            Err(SquareIndexError::new(value))
        }
    }
}

impl Into<Bitboard> for Square {
    fn into(self) -> Bitboard {
        Bitboard(0x1 << self as usize)
    }
}

impl Add<i8> for Square {
    type Output = i8;

    fn add(self, rhs: i8) -> Self::Output {
        let mailbox_idx = self.mailbox_idx() as i8 + rhs;
        MAILBOX[mailbox_idx as usize]
    }
}

impl Square {
    pub fn from_mailbox_no(mailbox_no: i8) -> Self {
        SQUARES[mailbox_no as usize]
    }

    pub fn from_bitboard(bb: Bitboard) -> Square {
        SQUARES[bb.0 as usize]
    }

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
            // TODO (tcd 6/2/23): better error here
            return Err(InvalidCharError::new('x'));
        }

        let f = File::try_from(chars[0])?;
        let r = Rank::try_from(chars[1])?;

        Ok(Some(Square::new(f, r)))
    }

    pub fn mailbox_idx(self) -> usize {
        MAILBOX_IDX[self as usize]
    }
}

#[repr(i8)]
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North = 10,
    NorthEast = 11,
    East = 1,
    SouthEast = -9,
    South = -10,
    SouthWest = -11,
    West = -1,
    NorthWest = 9,
    NorthEastL = 21,
    NorthWestL = 19,
    EastNorthL = 12,
    EastSouthL = -8,
    SouthEastL = -19,
    SouthWestL = -21,
    WestSouthL = -12,
    WestNorthL = 8,
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

// bits = [ wK, wQ, bK, bQ ]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastlingRights(pub u8);

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

    pub fn white_kingside(&self) -> bool {
        &self.0 & 0b1 == 0b1
    }

    pub fn white_queenside(&self) -> bool {
        (&self.0 >> 1) & 0b1 == 0b1
    }

    pub fn black_kingside(&self) -> bool {
        (&self.0 >> 2) & 0b1 == 0b1
    }

    pub fn black_queenside(&self) -> bool {
        (&self.0 >> 3) & 0b1 == 0b1
    }

    pub fn unset_white_bits(&mut self) {
        self.0 &= 0b1100;
    }

    pub fn unset_black_bits(&mut self) {
        self.0 &= 0b0011
    }

    pub fn all() -> CastlingRights {
        CastlingRights(0b1111)
    }

    pub fn empty() -> CastlingRights {
        CastlingRights(0b0000)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::play::utils::set_bits;

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
    fn test_square_mailbox_idx() {
        let first_idx = Square::A1.mailbox_idx();
        let first_mailbox_no = MAILBOX[first_idx];
        assert_eq!(SQUARES[first_mailbox_no as usize], Square::A1);

        let last_idx = Square::H8.mailbox_idx();
        let last_mailbox_no = MAILBOX[last_idx];
        assert_eq!(SQUARES[last_mailbox_no as usize], Square::H8);
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

    #[test]
    fn test_castling_rights_getters() {
        let white_queenside = "Q";
        let all = "KQkq";

        let empty_rights = CastlingRights::from_fen("").unwrap();
        let wq = CastlingRights::from_fen(white_queenside).unwrap();
        let all_rights = CastlingRights::from_fen(all).unwrap();

        assert!(
            !empty_rights.white_kingside()
                & !empty_rights.white_queenside()
                & !empty_rights.black_kingside()
                & !empty_rights.black_queenside()
        );
        assert!(
            !wq.white_kingside()
                & wq.white_queenside()
                & !wq.black_kingside()
                & !wq.black_queenside()
        );
        assert!(
            all_rights.white_kingside()
                & all_rights.white_queenside()
                & all_rights.black_kingside()
                & all_rights.black_queenside()
        );
    }

    #[test]
    fn test_castling_rights_unset_bits() {
        let mut empty_rights = CastlingRights::empty();
        let mut wq = CastlingRights::from_fen("Q").unwrap();
        let mut all_rights = CastlingRights::all();

        // essential to ensure that the unsetting is idempotent
        // b/c initially I didn't do that and it threw overflow errors (subtracting to negatives)
        empty_rights.unset_black_bits();
        assert_eq!(empty_rights.0, CastlingRights::empty().0);
        empty_rights.unset_white_bits();
        assert_eq!(empty_rights.0, CastlingRights::empty().0);

        wq.unset_white_bits();
        assert_eq!(wq.0, CastlingRights::empty().0);
        wq.unset_white_bits();
        assert_eq!(wq.0, CastlingRights::empty().0);
        wq.unset_black_bits();
        assert_eq!(wq.0, CastlingRights::empty().0);

        all_rights.unset_black_bits();
        assert_eq!(all_rights.black_kingside(), false);
        assert_eq!(all_rights.black_queenside(), false);
        assert_eq!(all_rights.white_queenside(), true);
        assert_eq!(all_rights.white_kingside(), true);
        all_rights.unset_black_bits();
        assert_eq!(all_rights.black_kingside(), false);
        assert_eq!(all_rights.black_queenside(), false);
        assert_eq!(all_rights.white_queenside(), true);
        assert_eq!(all_rights.white_kingside(), true);
    }

    #[test]
    fn test_direction_valid_moves() {
        // move a rook on b5 to the left
        let mailbox_no = Square::B5 + Direction::West as i8;
        assert!(mailbox_no >= 0);
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::A5);

        // move a knight on d5
        let mailbox_no = Square::D5 + Direction::NorthWestL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::C7);

        let mailbox_no = Square::D5 + Direction::NorthEastL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::E7);

        let mailbox_no = Square::D5 + Direction::EastNorthL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::F6);

        let mailbox_no = Square::D5 + Direction::WestNorthL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::B6);

        let mailbox_no = Square::D5 + Direction::WestSouthL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::B4);

        let mailbox_no = Square::D5 + Direction::SouthWestL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::C3);

        let mailbox_no = Square::D5 + Direction::SouthEastL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::E3);

        let mailbox_no = Square::D5 + Direction::EastSouthL as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::F4);

        // move a king on b2
        let mailbox_no = Square::B2 + Direction::North as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::B3);

        let mailbox_no = Square::B2 + Direction::NorthWest as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::A3);

        let mailbox_no = Square::B2 + Direction::West as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::A2);

        let mailbox_no = Square::B2 + Direction::SouthWest as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::A1);

        let mailbox_no = Square::B2 + Direction::South as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::B1);

        let mailbox_no = Square::B2 + Direction::SouthEast as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::C1);

        let mailbox_no = Square::B2 + Direction::East as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::C2);

        let mailbox_no = Square::B2 + Direction::NorthEast as i8;
        let sq = SQUARES[mailbox_no as usize];
        assert_eq!(sq, Square::C3);
    }

    #[test]
    fn test_direction_offboard_moves() {
        // move a rook on a5 to the left
        let mailbox_no = Square::A5 + Direction::West as i8;
        assert!(mailbox_no < 0);

        // move a rook on h6 to the right
        let mailbox_no = Square::H6 + Direction::East as i8;
        assert!(mailbox_no < 0);

        // move a knight on a1 down
        let mailbox_no = Square::A1 + Direction::SouthEastL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::A1 + Direction::SouthWestL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::A1 + Direction::WestSouthL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::A1 + Direction::EastSouthL as i8;
        assert!(mailbox_no < 0);

        // move a knight on a6 to the left
        let mailbox_no = Square::A6 + Direction::NorthWestL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::A6 + Direction::SouthWestL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::A6 + Direction::WestNorthL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::A6 + Direction::WestSouthL as i8;
        assert!(mailbox_no < 0);

        // move a knight on h6 to the right
        let mailbox_no = Square::H6 + Direction::NorthEastL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::H6 + Direction::SouthEastL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::H6 + Direction::EastNorthL as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::H6 + Direction::EastSouthL as i8;
        assert!(mailbox_no < 0);

        // move a queen on d8 up
        let mailbox_no = Square::D8 + Direction::North as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::D8 + Direction::NorthWest as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::D8 + Direction::NorthEast as i8;
        assert!(mailbox_no < 0);

        // move a queen on d1 down
        let mailbox_no = Square::D1 + Direction::South as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::D1 + Direction::SouthWest as i8;
        assert!(mailbox_no < 0);
        let mailbox_no = Square::D1 + Direction::SouthEast as i8;
        assert!(mailbox_no < 0);

        // move a bishop on h8 up
        let mailbox_no = Square::H8 + Direction::NorthEast as i8;
        assert!(mailbox_no < 0);

        // move a bishop on a8 up
        let mailbox_no = Square::A8 + Direction::NorthWest as i8;
        assert!(mailbox_no < 0);

        // move a bishop on h1 down
        let mailbox_no = Square::H1 + Direction::SouthEast as i8;
        assert!(mailbox_no < 0);

        // move a bishop on a1 down
        let mailbox_no = Square::A1 + Direction::SouthWest as i8;
        assert!(mailbox_no < 0);
    }
}
