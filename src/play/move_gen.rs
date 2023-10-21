use super::{
    constants::DIRECTIONS,
    position::Position,
    r#move::{Move, MoveList},
    types::{Color, Piece, Square},
    utils,
};

pub struct MoveGenerator {}

impl MoveGenerator {
    // TODO: figure out how this can return MoveList as opposed to mutating it
    // TODO: add piece type that's agnostic of color
    pub fn generate_moves(
        position: &Position,
        piece: Piece,
        sq: Square,
        color: Color,
        moves: &mut MoveList,
    ) {
        if piece.can_slide() {
            let dirs = &DIRECTIONS[piece.attack_direction_idx()].to_vec();
            let color = piece.color();
            utils::recur_move_search(&position.board, color, dirs, moves, sq, 1);
        }

        if (piece == Piece::BlackKnight) | (piece == Piece::WhiteKnight) {
            let dirs = &DIRECTIONS[piece.attack_direction_idx()];
            for dir in dirs {
                let target_sq_mailbox_no = sq + *dir as i8;
                if target_sq_mailbox_no >= 0 {
                    let other_sq = Square::from_mailbox_no(target_sq_mailbox_no);
                    if !position.board.sq_taken_by_color(other_sq, piece.color()) {
                        let captured = position.board.piece(&other_sq);
                        moves.push(Move::new(sq, other_sq, captured, None, false, false, false));
                    }
                }
            }
        }
    }
}
