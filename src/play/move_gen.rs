use super::{
    constants::DIRECTIONS,
    position::Position,
    r#move::{Move, MoveList},
    types::{Color, Direction, Piece, PieceType, Square},
    utils,
};

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate_pawn_moves(position: &Position, sq: Square, moves: &mut MoveList) {
        let promo_piece = Piece::of(PieceType::Queen, position.side_to_move);
        let ep_captured = Piece::of(PieceType::Pawn, position.side_to_move.opposing());
        let fwd_mailbox_no = sq + position.side_to_move.pawn_push_dir() as i8;

        // pawn start
        if sq.rank() == position.side_to_move.pawn_start_rank() {
            let sq_2_in_front =
                Square::from_mailbox_no(sq + 2 * position.side_to_move.pawn_push_dir() as i8);
            if (!position
                .board
                .sq_taken(Square::from_mailbox_no(fwd_mailbox_no)))
                & (!position.board.sq_taken(sq_2_in_front))
            {
                moves.push(Move::new(sq, sq_2_in_front, None, None, false, true, false));
            }
        }

        // normal forward & promotion
        if fwd_mailbox_no >= 0 {
            let fwd_sq = Square::from_mailbox_no(fwd_mailbox_no);
            let promoted = if sq.rank() == position.side_to_move.pawn_promo_rank() {
                Some(promo_piece)
            } else {
                None
            };
            if !position.board.sq_taken(fwd_sq) {
                moves.push(Move::new(sq, fwd_sq, None, promoted, false, false, false));
            }
        }

        for dir in position.side_to_move.pawn_diagonals() {
            let diag_mailbox_no = sq + *dir as i8;
            if diag_mailbox_no >= 0 {
                let diag_sq = Square::from_mailbox_no(diag_mailbox_no);
                if position
                    .board
                    .sq_taken_by_color(diag_sq, position.side_to_move.opposing())
                {
                    let captured = position.board.piece(&diag_sq);
                    let promoted = if sq.rank() == position.side_to_move.pawn_promo_rank() {
                        Some(promo_piece)
                    } else {
                        None
                    };
                    moves.push(Move::new(
                        sq, diag_sq, captured, promoted, false, false, false,
                    ));
                } else {
                    if let Some(ep_sq) = position.en_passant {
                        if diag_sq == ep_sq {
                            moves.push(Move::new(
                                sq,
                                diag_sq,
                                Some(ep_captured),
                                None,
                                true,
                                false,
                                false,
                            ))
                        }
                    }
                }
            }
        }
    }

    // TODO: figure out how this can return MoveList as opposed to mutating it
    // this can just take a piece type
    pub fn generate_moves(position: &Position, piece: Piece, sq: Square, moves: &mut MoveList) {
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
                        moves.push(Move::new(sq, other_sq, captured, None, false, false, false));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pawn_moves_pawn_start() {
        let pos = Position::default();
        let moves = &mut MoveList::empty();

        MoveGenerator::generate_pawn_moves(&pos, Square::A2, moves);

        let expected = vec![
            Move::new(Square::A2, Square::A3, None, None, false, false, false),
            Move::new(Square::A2, Square::A4, None, None, false, true, false),
        ];
        assert_eq!(
            moves
                .sorted()
                .filter(|mv| !mv.is_placeholder())
                .collect::<Vec<Move>>(),
            expected
        );

        // TODO: add black pawns
    }

    #[test]
    fn test_generate_pawn_moves_forward() {}

    #[test]
    fn test_generate_pawn_moves_promotion() {}

    #[test]
    fn test_generate_pawn_moves_capture() {}

    #[test]
    fn test_generate_pawn_moves_capture_promotion() {}

    #[test]
    fn test_generate_pawn_moves_en_passant_capture() {}
}
