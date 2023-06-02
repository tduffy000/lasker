use std::fmt::Debug;

pub(in crate::play) mod board;
pub(in crate::play) mod constants;
mod error;
pub mod key;
pub mod r#move;
pub(in crate::play) mod position;
pub(in crate::play) mod types;
mod utils;

use error::FENParsingError;

use self::position::Position;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameState {
    pub position: Position,
    pub fifty_move_counter: u8,
    pub fifty_move_country_hist: Vec<u8>,
    pub ply: u8,
    pub history_ply: u8,
    pub position_key: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            position: Position::default(),
            fifty_move_counter: 0,
            fifty_move_country_hist: vec![],
            ply: 0,
            history_ply: 0,
            position_key: Default::default(),
        }
    }
}

impl GameState {
    pub fn print_board(&self) {
        print!("{:?}", self.position.board)
    }

    pub fn from_fen(fen: impl ToString) -> Result<GameState, FENParsingError> {
        let mut state = GameState::default();

        let n_fields = 6;
        let fen_str = fen.to_string();
        if fen_str.split(' ').count() != n_fields {
            return Err(FENParsingError::new(" "));
        }
        let fields: Vec<String> = fen_str.split(' ').map(|s| s.to_string()).collect();

        // board
        state.position = Position::from_fields(fields[..4].to_vec())?;

        // TODO: parse plys + move clocks

        Ok(state)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_game_state_from_fen() {
        let start_state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed_state = GameState::from_fen(start_state).unwrap();
        assert_eq!(parsed_state, GameState::default());
    }

}
