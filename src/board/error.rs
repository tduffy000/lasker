#[derive(Debug)]
pub struct SquareIndexError {
    idx: usize,
    msg: String,
}

impl SquareIndexError {
    pub fn new(idx: usize, msg: impl ToString) -> Self {
        SquareIndexError { idx, msg: msg.to_string() }
    }
}
