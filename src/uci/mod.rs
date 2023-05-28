use std::io::{self, stdin};

use crate::{board::BoardState, perft::run_perft};

fn handle_position(mut buf: String) -> BoardState {
    let mut whitespace_it = buf.split_ascii_whitespace();
    whitespace_it.next(); // consume position
    if let Some(sub) = whitespace_it.next() {
        if sub == "fen" {
            let fen_str = whitespace_it.fold(String::new(), |mut acc, el| {
                acc.push_str(" ");
                acc.push_str(el);
                acc
            }).trim().to_string();
            BoardState::from_fen(fen_str).unwrap()
        } else {
            BoardState::default()
        }
    } else {
        BoardState::default()
    }
}

fn handle_perft(buf: String, pos: &BoardState) {
    if let Some(depth_str) = buf.split_ascii_whitespace().next_back() {
        let depth = depth_str.parse().unwrap();
        let nodes_searched = run_perft(*pos, depth);
        println!("\ntotal nodes searched: {nodes_searched}");
    }
}

pub fn uci_loop() -> Result<(), io::Error> {

    let stdin = stdin();
    let mut pos = BoardState::default();

    loop {

        let mut buf = String::new();
        let _ = stdin.read_line(&mut buf);

        if buf.starts_with("position") {
            pos = handle_position(buf);
        } else if buf.starts_with("go perft") {
            handle_perft(buf, &pos);
        }

    }
}