use crate::play::types::Square;

pub struct MoveError{err: MoveErrorType}

pub enum MoveErrorType {
    SquareTakenError(Square),
    NoPieceOnSquareError(Square),
}

impl MoveError {
    pub fn new(err: MoveErrorType) -> MoveError {
        MoveError {err}
    }

    pub fn print_msg(&self) {
        let msg = match self.err {
            MoveErrorType::SquareTakenError(sq) => 
                format!("Square: {} is already taken", sq),
            MoveErrorType::NoPieceOnSquareError(sq) => 
                format!("No piece on square: {}", sq),
        };
        eprintln!("{}", msg)
    }
}

#[derive(Debug)]
pub struct SquareIndexError {
    idx: usize,
}

impl SquareIndexError {
    pub fn new(idx: usize) -> Self {
        SquareIndexError { idx }
    }
}

#[derive(Debug)]
pub struct InvalidCharError {
    ch: char,
}

impl InvalidCharError {
    pub fn new(ch: char) -> Self {
        InvalidCharError { ch }
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

impl From<InvalidCharError> for FENParsingError {
    fn from(_: InvalidCharError) -> Self {
        FENParsingError::new("")
    }
}

impl From<SquareIndexError> for FENParsingError {
    fn from(_: SquareIndexError) -> Self {
        FENParsingError::new("")
    }
}
