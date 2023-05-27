mod perft_parser;

use std::io::{self, stdin};
use perft_parser::handle_perft;

pub fn uci_loop() -> Result<(), io::Error> {

    let mut buf = String::new();
    let stdin = stdin();

    loop {

        let _ = stdin.read_line(&mut buf)?;

        if buf.to_ascii_lowercase().starts_with("go perft") {
            handle_perft(&mut buf)
        }

        buf.clear()
    }
}