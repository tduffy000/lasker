use crate::{perft::run_perft, board::BoardState};

/// go perft <fen> <depth>
pub(in crate::uci) fn handle_perft(s: &mut String) {
    let mut white_space_it = s.split_ascii_whitespace();
    
    // consume go + perft
    let _ = white_space_it.next();
    let _ = white_space_it.next();

    // remove depth from the end
    let mb_depth_str = white_space_it.next_back();
    let mb_fen = white_space_it.fold(String::new(), |mut acc, el| {
        acc.push_str(" ");
        acc.push_str(el);
        acc
    }).trim().to_string();

    if let Some(depth_str) = mb_depth_str {
        let depth = depth_str.parse().unwrap();
        let state = BoardState::from_fen(mb_fen).unwrap();
        run_perft(state, depth);
    }
}