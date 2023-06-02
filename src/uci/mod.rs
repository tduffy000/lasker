use std::io::{self, stdin};

use crate::{perft::run_perft, play::GameState};

fn handle_position(mut buf: String) -> GameState {
    let mut whitespace_it = buf.split_ascii_whitespace();
    whitespace_it.next(); // consume position
    if let Some(sub) = whitespace_it.next() {
        if sub == "fen" {
            let fen_str = whitespace_it
                .fold(String::new(), |mut acc, el| {
                    acc.push_str(" ");
                    acc.push_str(el);
                    acc
                })
                .trim()
                .to_string();
            GameState::from_fen(fen_str).unwrap()
        } else {
            GameState::default()
        }
    } else {
        GameState::default()
    }
}

fn handle_perft(buf: String, pos: &mut GameState) {
    if let Some(depth_str) = buf.split_ascii_whitespace().next_back() {
        println!("\nevaluating position: {:?}", pos);
        let depth = depth_str.parse().unwrap();
        let nodes_searched = run_perft(pos, depth);
        println!("\ntotal nodes searched: {nodes_searched}");
    }
}

pub fn uci_loop() -> Result<(), io::Error> {
    let stdin = stdin();
    let mut pos = GameState::default();

    loop {
        let mut buf = String::new();
        let _ = stdin.read_line(&mut buf);

        if buf.starts_with("position") {
            pos = handle_position(buf);
        } else if buf.starts_with("go perft") {
            handle_perft(buf, &mut pos);
        }
    }
}
