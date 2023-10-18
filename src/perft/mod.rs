use crate::play::{
    r#move::{make_move, unmake_move},
    GameState,
};

pub fn run_perft(state: &mut GameState, depth: u64) -> u64 {
    state
        .position
        .legal_moves()
        .map(|mv| {
            let mut s = state.clone();
            match make_move(mv, &mut s) {
                Ok(()) => {}
                Err(e) => {
                    println!("error making move: {:?}", mv.to_string());
                    e.print_msg()
                },
            };
            let nodes: u64 = perft(&mut s, depth - 1);
            println!("{mv}: {nodes}");
            nodes
        })
        .fold(0, |acc, x| acc + x)
}

fn perft(state: &mut GameState, depth: u64) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut nodes: u64 = 0;
    for mv in state.position.legal_moves() {
        match make_move(mv, state) {
            Ok(()) => {}
            Err(e) => {
                println!("\n\rerror making move: {:?}", mv.to_string());
                println!("board: {:?}", state.position.board);
                e.print_msg()
            },
        };
        nodes += perft(state, depth - 1);
        match unmake_move(mv, state) {
            Ok(()) => {}
            Err(e) => {
                println!("\n\rerror unmaking move: {:?}", mv.to_string());
                println!("board: {:?}", state.position.board);
                e.print_msg()
            },
        };
    }
    nodes
}
