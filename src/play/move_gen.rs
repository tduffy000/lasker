use super::{r#move::MoveList, types::{Color, Square, Piece}, constants::DIRECTIONS, utils, position::Position};

pub struct MoveGenerator {}

impl MoveGenerator {
    // TODO: figure out how this can return MoveList as opposed to mutating it
    pub fn generate_moves(
        position: &Position,
        piece: Piece,
        sq: Square,
        color: Color,
        moves: &mut MoveList
    ) {
        if piece.can_slide() {
            let dirs = &DIRECTIONS[piece.attack_direction_idx()].to_vec();
            let color = piece.color();
            utils::recur_move_search(&position.board, color, dirs, moves, sq, 1);
        }


    }
}