use crate::board::types::Square;

#[derive(Debug)]
pub struct SquareIndexError {
    idx: usize,
    msg: String,
}

impl SquareIndexError {
    pub fn new(idx: usize, msg: impl ToString) -> Self {
        SquareIndexError {
            idx,
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SquareTakenError {
    square: Square,
    msg: String,
}

impl SquareTakenError {
    pub fn new(square: Square) -> Self {
        SquareTakenError {
            square,
            msg: "That square is taken!".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct NoPieceOnSquareError {
    square: Square,
    msg: String,
}

impl NoPieceOnSquareError {
    pub fn new(square: Square) -> Self {
        NoPieceOnSquareError {
            square,
            msg: "There's no piece on that square!".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct InvalidPieceCharError {
    ch: char,
}

impl InvalidPieceCharError {
    pub fn new(ch: char) -> Self {
        InvalidPieceCharError { ch }
    }
}

#[derive(Debug)]
pub struct InvalidCastlingRightCharError {
    ch: char,
}

impl InvalidCastlingRightCharError {
    pub fn new(ch: char) -> Self {
        InvalidCastlingRightCharError { ch }
    }
}

#[derive(Debug)]
pub struct FENParsingError {
    msg: String,
}

impl FENParsingError {
    pub fn new(msg: impl ToString) -> Self {
        FENParsingError {
            msg: msg.to_string(),
        }
    }
}

impl From<SquareIndexError> for FENParsingError {
    fn from(_: SquareIndexError) -> Self {
        FENParsingError::new("")
    }
}

impl From<InvalidPieceCharError> for FENParsingError {
    fn from(_: InvalidPieceCharError) -> Self {
        FENParsingError::new("")
    }
}

impl From<InvalidCastlingRightCharError> for FENParsingError {
    fn from(_: InvalidCastlingRightCharError) -> Self {
        FENParsingError::new("")
    }
}
