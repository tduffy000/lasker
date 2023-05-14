use std::env;

use board::key::PositionKeyGenerator;
use board::BoardState;
use perft::run_perft;

mod board;
mod perft;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "perft" {
        let state = BoardState::from_fen(args[2].clone()).unwrap();
        let depth = args[3].parse::<u64>().unwrap();
        let result = run_perft(state, depth);
        println!("\nperft: {result}");
    } else {
        let key_manager: PositionKeyGenerator = PositionKeyGenerator::new();
        let init_state = BoardState::default();
        init_state.print_board();
    }
}
