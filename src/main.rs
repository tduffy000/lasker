use board::key::PositionKeyGenerator;
use board::BoardState;

mod board;
mod perft;

fn main() {
    let key_manager: PositionKeyGenerator = PositionKeyGenerator::new();
    let init_state = BoardState::default();
    init_state.print_board();
}
