use board::key::PositionKeyGenerator;
use board::BoardState;

mod board;

fn main() {
    let key_manager: PositionKeyGenerator = PositionKeyGenerator::new();
    let mut state = BoardState::default();

    let moves = state.legal_moves();
    for mv in moves {
        state.make_move(mv);
        state.print_board();
        state.unmake_move(mv);
    }
}
