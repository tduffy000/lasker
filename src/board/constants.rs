use crate::board::types::{Color, Direction, File, Piece, Rank, Square};

// using Little-Endian Rank File Mapping
// @see https://www.chessprogramming.org/Square_Mapping_Considerations
pub const FILE_A: u64 = 0x0101010101010101;
pub const RANK_1: u64 = 0xFF;
pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
pub const H1_A1_DIAGONAL: u64 = 0x0102040810204080;
const WHITE_SQUARES: u64 = 0x55AA55AA55AA55AA;
const BLACK_SQUARES: u64 = 0xAA55AA55AA55AA55;

pub const COLORS: [Color; 2] = [Color::White, Color::Black];

pub const DIRECTIONS: [Direction; 16] = [
    Direction::North,
    Direction::East,
    Direction::West,
    Direction::South,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthEast,
    Direction::NorthWest,
    Direction::NorthEastL,
    Direction::NorthWestL,
    Direction::EastNorthL,
    Direction::EastSouthL,
    Direction::SouthEastL,
    Direction::SouthWestL,
    Direction::WestSouthL,
    Direction::WestNorthL,
];

pub const DIAGONAL_DIRECTIONS: [Direction; 4] = [
    Direction::SouthEast,
    Direction::NorthEast,
    Direction::SouthWest,
    Direction::NorthWest,
];

pub const STRAIGHT_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::West,
    Direction::South,
];

pub const KNIGHT_DIRECTIONS: [Direction; 8] = [
    Direction::NorthEastL,
    Direction::NorthWestL,
    Direction::EastNorthL,
    Direction::EastSouthL,
    Direction::SouthEastL,
    Direction::SouthWestL,
    Direction::WestSouthL,
    Direction::WestNorthL,
];

pub const PIECES: [Piece; 12] = [
    Piece::WhitePawn,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteRook,
    Piece::WhiteQueen,
    Piece::WhiteKing,
    Piece::BlackPawn,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackRook,
    Piece::BlackQueen,
    Piece::BlackKing,
];

pub const WHITE_PIECES: [Piece; 6] = [
    Piece::WhitePawn,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteRook,
    Piece::WhiteQueen,
    Piece::WhiteKing,
];

pub const BLACK_PIECES: [Piece; 6] = [
    Piece::BlackPawn,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackRook,
    Piece::BlackQueen,
    Piece::BlackKing,
];

pub const PIECE_VALUES: [u32; 12] = [
    100, 325, 325, 550, 1000, 50000, 100, 325, 325, 550, 1000, 50000,
];

pub const IS_MINOR_PIECE: [bool; 12] = [
    false, true, true, false, false, false, false, true, true, false, false, false,
];

pub const IS_MAJOR_PIECE: [bool; 12] = [
    false, false, false, true, true, false, false, false, false, true, true, false,
];

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

pub const MAILBOX: [i8; 120] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3,
    4, 5, 6, 7, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15, -1, -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, 32, 33, 34, 35, 36, 37, 38, 39, -1, -1, 40, 41, 42,
    43, 44, 45, 46, 47, -1, -1, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62,
    63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub const MAILBOX_IDX: [usize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68, 71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88, 91, 92, 93, 94, 95, 96, 97, 98,
];
