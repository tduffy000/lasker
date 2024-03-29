use rand::random;

use crate::play::{constants::SQUARES, types::Color, GameState};

pub struct PositionKeyGenerator {
    key: u64,
    piece_hashes: [[u64; 64]; 12],
    en_passant_hashes: [u64; 64],
    side_to_move_hash: u64,
    castling_permission_hashes: [u64; 16], // 4!
}

impl PositionKeyGenerator {
    pub fn new() -> Self {
        // pieces
        let mut p = [[0; 64]; 12];
        for i in 0..p.len() {
            for j in 0..p[i].len() {
                p[i][j] = random::<u64>();
            }
        }
        // castling
        let mut c = [0; 16];
        for i in 0..c.len() {
            c[i] = random::<u64>();
        }
        // en passant
        let mut e = [0; 64];
        for i in 0..e.len() {
            e[i] = random::<u64>();
        }

        PositionKeyGenerator {
            key: 0,
            piece_hashes: p,
            en_passant_hashes: e,
            side_to_move_hash: random::<u64>(),
            castling_permission_hashes: c,
        }
    }

    pub fn key(&self) -> u64 {
        self.key
    }

    // this should be incremental, i.e. only based on the
    // pieces that change
    pub fn hash_board(&self, state: &GameState) -> u64 {
        let mut key = 0;

        // pieces
        for sq in SQUARES.iter() {
            if let Some(piece) = state.position.board.piece(sq) {
                let piece_idx = piece as usize;
                key ^= self.piece_hashes[piece_idx][*sq as usize];
            }
        }

        // castling
        key ^= self.castling_permission_hashes[state.position.castling_permissions.0 as usize];

        // en passant
        if let Some(sq) = state.position.en_passant {
            key ^= self.en_passant_hashes[sq as usize]
        }

        // to move
        if state.position.side_to_move == Color::White {
            key ^= self.side_to_move_hash
        }

        key
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::play::types::{CastlingRights, Piece, Square};

    #[test]
    fn test_hash_board() {
        let mut state = GameState::default();
        let key_gen = PositionKeyGenerator::new();

        let base_key = key_gen.hash_board(&state);

        // switch colors
        state.position.side_to_move = Color::Black;
        assert_ne!(base_key, key_gen.hash_board(&state));
        state.position.side_to_move = Color::White;
        assert_eq!(base_key, key_gen.hash_board(&state));

        // switch en passant
        state.position.en_passant = Some(Square::C3);
        assert_ne!(base_key, key_gen.hash_board(&state));
        state.position.en_passant = None;
        assert_eq!(base_key, key_gen.hash_board(&state));

        // switch castling rights (default == 0b1111)
        state.position.castling_permissions = CastlingRights(0b1010);
        assert_ne!(base_key, key_gen.hash_board(&state));
        state.position.castling_permissions = CastlingRights(0b1111);
        assert_eq!(base_key, key_gen.hash_board(&state));

        // add a piece
        let _ = state
            .position
            .board
            .add_piece(Piece::BlackQueen, Square::A4);
        assert_ne!(base_key, key_gen.hash_board(&state));
        let _ = state.position.board.remove_piece(Square::A4);
        assert_eq!(base_key, key_gen.hash_board(&state));
    }
}
