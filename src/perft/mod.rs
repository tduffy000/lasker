use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::board::BoardState;

#[derive(Debug)]
struct ParsedPerft {
    fen: String,
    depth: u64,
    distinct_moves: u64,
}

impl ParsedPerft {
    pub fn from_line(s: String) -> ParsedPerft {
        let n_fields = 3;
        let fields: Vec<&str> = s.split(",").collect();
        let fen = fields[0].to_string();
        let depth = fields[1].parse::<u64>().unwrap();
        let distinct_moves = fields[2].parse::<u64>().unwrap();
        ParsedPerft {
            fen,
            depth,
            distinct_moves,
        }
    }
}

// TODO: would be useful to print perft result for each sub (depth-1) move.
pub fn perft(state: BoardState, depth: u64) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut nodes: u64 = 0;
    for mv in state.legal_moves() {
        nodes += perft(state.make_move(mv), depth - 1);
    }
    nodes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_1_test() {
        let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1,1,20".to_string();
        let parsed = ParsedPerft::from_line(s);
        let state = BoardState::from_fen(parsed.fen).unwrap();
        let result = perft(state, parsed.depth);
        assert_eq!(result, parsed.distinct_moves);
    }

    #[test]
    fn perft_2_test() {
        let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1,2,400".to_string();
        let parsed = ParsedPerft::from_line(s);
        let state = BoardState::from_fen(parsed.fen).unwrap();
        let result = perft(state, parsed.depth);
        assert_eq!(result, parsed.distinct_moves);
    }

    #[test]
    fn perft_3_test() {
        let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1,3,8902".to_string();
        let parsed = ParsedPerft::from_line(s);
        let state = BoardState::from_fen(parsed.fen).unwrap();
        let result = perft(state, parsed.depth);
        assert_eq!(result, parsed.distinct_moves);
    }

    #[test]
    fn perft_4_test() {
        let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1,4,197281".to_string();
        let parsed = ParsedPerft::from_line(s);
        let state = BoardState::from_fen(parsed.fen).unwrap();
        let result = perft(state, parsed.depth);
        assert_eq!(result, parsed.distinct_moves);
    }

    #[test]
    fn perft_5_test() {
        let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1,5,4865609".to_string();
        let parsed = ParsedPerft::from_line(s);
        let state = BoardState::from_fen(parsed.fen).unwrap();
        let result = perft(state, parsed.depth);
        assert_eq!(result, parsed.distinct_moves);
    }

}
