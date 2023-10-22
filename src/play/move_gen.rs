use super::{
    constants::DIRECTIONS,
    position::Position,
    r#move::{Move, MoveList},
    types::{Color, Piece, Square, PieceType},
    utils,
};

pub struct MoveGenerator {}

impl MoveGenerator {
    // TODO: figure out how this can return MoveList as opposed to mutating it
    pub fn generate_moves(
        position: &Position,
        piece: Piece,
        sq: Square,
        moves: &mut MoveList,
    ) {
        if piece.can_slide() {
            let dirs = &DIRECTIONS[piece.attack_direction_idx()].to_vec();
            let color = piece.color();
            utils::recur_move_search(&position.board, color, dirs, moves, sq, 1);
        }

        if piece.piece_type() == PieceType::Knight {
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

        if piece.piece_type() == PieceType::King {
            let dirs = &DIRECTIONS[piece.attack_direction_idx()];
            for dir in dirs {
                let target_sq_mailbox_no = sq + *dir as i8;
                if target_sq_mailbox_no >= 0 {
                    let other_sq = Square::from_mailbox_no(target_sq_mailbox_no);
                    if !position.board.sq_taken_by_color(other_sq, piece.color())
                        & !position.is_square_attacked(other_sq, piece.opposing_color())
                    {
                        let captured = position.board.piece(&other_sq);
                        moves.push(Move::new(
                            sq, other_sq, captured, None, false, false, false,
                        ));
                    }
                }
            }

            match piece.color() {
                Color::White => {
                    if position.castling_permissions.white_kingside()
                        & !position.board.sq_taken(Square::F1)
                        & !position.board.sq_taken(Square::G1)
                        & !position.is_square_attacked(Square::F1, Color::Black)
                        & !position.is_square_attacked(Square::G1, Color::Black)
                    {
                        moves.push(Move::new(
                            Square::E1,
                            Square::G1,
                            None,
                            None,
                            false,
                            false,
                            true,
                        ));
                    }
                    if position.castling_permissions.white_queenside()
                        & !position.board.sq_taken(Square::C1)
                        & !position.board.sq_taken(Square::D1)
                        & !position.is_square_attacked(Square::C1, Color::Black)
                        & !position.is_square_attacked(Square::D1, Color::Black)
                    {
                        moves.push(Move::new(
                            Square::E1,
                            Square::C1,
                            None,
                            None,
                            false,
                            false,
                            true,
                        ));
                    }
                }
                Color::Black => {
                    if position.castling_permissions.black_kingside()
                        & !position.board.sq_taken(Square::F8)
                        & !position.board.sq_taken(Square::G8)
                        & !position.is_square_attacked(Square::F8, Color::White)
                        & !position.is_square_attacked(Square::G8, Color::White)
                    {
                        moves.push(Move::new(
                            Square::E8,
                            Square::G8,
                            None,
                            None,
                            false,
                            false,
                            true,
                        ));
                    }
                    if position.castling_permissions.black_queenside()
                        & !position.board.sq_taken(Square::C8)
                        & !position.board.sq_taken(Square::D8)
                        & !position.is_square_attacked(Square::C8, Color::White)
                        & !position.is_square_attacked(Square::D8, Color::White)
                    {
                        moves.push(Move::new(
                            Square::E8,
                            Square::C8,
                            None,
                            None,
                            false,
                            false,
                            true,
                        ));
                    }
                }
            }
        }
    }
}
