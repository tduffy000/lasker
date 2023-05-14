use crate::board::BoardState;

pub fn run_perft(state: BoardState, depth: u64) -> u64 {
    state
        .legal_moves()
        .map(|mv| {
            let nodes: u64 = perft(state.make_move(mv), depth - 1);
            println!("{mv}: {nodes}");
            nodes
        })
        .fold(0, |acc, x| acc + x)
}

fn perft(state: BoardState, depth: u64) -> u64 {
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
        let state =
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        let result = perft(state, 1);
        assert_eq!(result, 20);
    }

    #[test]
    fn perft_2_test() {
        let state =
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        let result = perft(state, 2);
        assert_eq!(result, 400);
    }
}
