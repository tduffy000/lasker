use super::{
    board::Board,
    constants::DIRECTIONS,
    position::Position,
    r#move::{Move, MoveList},
    types::{Color, Piece, PieceType, Square},
    utils,
};

fn move_alleviates_check(board: &Board, side_to_move: &Color, m: &Move) -> bool {
    let mut future_board = board.clone();
    if future_board.sq_taken_by_color(m.to_sq(), side_to_move.opposing()) {
        future_board.remove_piece(m.to_sq()).unwrap();
    }
    future_board.move_piece(m.from_sq(), m.to_sq()).unwrap();
    !future_board.is_king_in_check(*side_to_move)
}

// TODO: re-merge this with generate moves, caller doesn't care what piece it is
pub fn generate_pawn_moves(position: &Position, sq: Square, moves: &mut MoveList) {
    let promo_piece = Piece::of(PieceType::Queen, position.side_to_move);
    let ep_captured = Piece::of(PieceType::Pawn, position.side_to_move.opposing());
    let fwd_mailbox_no = sq + position.side_to_move.pawn_push_dir() as i8;

    let is_check = position.board.is_king_in_check(position.side_to_move);

    if position.board.is_square_pinned(&sq) & !is_check {
        return;
    }

    // pawn start
    if sq.rank() == position.side_to_move.pawn_start_rank() {
        let sq_2_in_front =
            Square::from_mailbox_no(sq + 2 * position.side_to_move.pawn_push_dir() as i8);

        if (!position
            .board
            .sq_taken(Square::from_mailbox_no(fwd_mailbox_no)))
            & (!position.board.sq_taken(sq_2_in_front))
        {
            let move_to_make = Move::new(sq, sq_2_in_front, None, None, false, true, false);
            if is_check {
                if move_alleviates_check(&position.board, &position.side_to_move, &move_to_make) {
                    moves.push(move_to_make);
                }
            } else {
                moves.push(move_to_make);
            }
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

        let move_to_make = Move::new(sq, fwd_sq, None, promoted, false, false, false);

        if !position.board.sq_taken(fwd_sq) {
            if is_check {
                if move_alleviates_check(&position.board, &position.side_to_move, &move_to_make) {
                    moves.push(move_to_make);
                }
            } else {
                moves.push(move_to_make);
            }
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
                let move_to_make = Move::new(sq, diag_sq, captured, promoted, false, false, false);
                if is_check {
                    if move_alleviates_check(&position.board, &position.side_to_move, &move_to_make)
                    {
                        moves.push(move_to_make);
                    }
                } else {
                    moves.push(move_to_make);
                }
            } else {
                // TODO (tcd 9/2/24): check allevation
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
pub fn generate_moves(position: &Position, piece: Piece, sq: Square, moves: &mut MoveList) {
    // TODO: use refactoring to consider if the piece is pinned (i.e. can't move)
    // OR if the king is currently in check (then it can ONLY move if king is not in check)

    if position.board.is_square_pinned(&sq) {
        return;
    }

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
                    & !position
                        .board
                        .is_square_attacked(other_sq, piece.opposing_color())
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
                    & !position.board.is_square_attacked(Square::F1, Color::Black)
                    & !position.board.is_square_attacked(Square::G1, Color::Black)
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
                    & !position.board.is_square_attacked(Square::C1, Color::Black)
                    & !position.board.is_square_attacked(Square::D1, Color::Black)
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
                    & !position.board.is_square_attacked(Square::F8, Color::White)
                    & !position.board.is_square_attacked(Square::G8, Color::White)
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
                    & !position.board.is_square_attacked(Square::C8, Color::White)
                    & !position.board.is_square_attacked(Square::D8, Color::White)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pawn_moves_pawn_start() {
        // white pawn start
        let white_pos = Position::default();
        let white_moves = &mut MoveList::empty();

        generate_pawn_moves(&white_pos, Square::A2, white_moves);
        let white_expected = vec![
            Move::new(Square::A2, Square::A3, None, None, false, false, false),
            Move::new(Square::A2, Square::A4, None, None, false, true, false),
        ];
        assert_eq!(
            white_moves
                .sorted()
                .filter(|mv| !mv.is_placeholder())
                .collect::<Vec<Move>>(),
            white_expected
        );

        // black pawn start
        let black_init_fen = "rnbqkbnr/ppp1pppp/8/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq -";
        let black_pos = Position::from_fen(black_init_fen).unwrap();
        let black_moves = &mut MoveList::empty();

        generate_pawn_moves(&black_pos, Square::E7, black_moves);
        let black_expected = vec![
            Move::new(Square::E7, Square::E6, None, None, false, false, false),
            Move::new(Square::E7, Square::E5, None, None, false, true, false),
        ];
        assert_eq!(
            black_moves
                .sorted()
                .filter(|mv| !mv.is_placeholder())
                .collect::<Vec<Move>>(),
            black_expected
        );
    }

    #[test]
    fn test_generate_pawn_moves_respect_block() {
        // pawn start attempt with blocker (black bishop on a3)
        let a2_blocked_fen = "rnbqk1nr/ppp2ppp/4p3/3p4/3PP3/b1N5/PPP2PPP/R1BQKBNR w KQkq -";
        let a2_blocked_pos = Position::from_fen(a2_blocked_fen).unwrap();
        let a2_moves = &mut MoveList::empty();

        generate_pawn_moves(&a2_blocked_pos, Square::A2, a2_moves);
        assert_eq!(a2_moves.count(), 0);

        // forward move with blocker
        let d5_blocked_fen = "rnbqk1nr/ppp2ppp/4p3/3pP2Q/3P4/b1N2P2/PPP3PP/R1B1KBNR b KQkq -";
        let d5_blocked_pos = Position::from_fen(d5_blocked_fen).unwrap();
        let d5_moves = &mut MoveList::empty();

        generate_pawn_moves(&d5_blocked_pos, Square::A2, d5_moves);
        assert_eq!(d5_moves.count(), 0);
    }

    #[test]
    fn test_generate_pawn_moves_respect_pin() {
        // pawn start attempt with pin
        let f7_pinned_fen = "rnbqk1nr/ppp2ppp/4p3/3p3Q/3PP3/b1N5/PPP2PPP/R1B1KBNR b KQkq -";
        let f7_pinned_pos = Position::from_fen(f7_pinned_fen).unwrap();
        let f7_moves = &mut MoveList::empty();

        generate_pawn_moves(&f7_pinned_pos, Square::F7, f7_moves);
        assert_eq!(f7_moves.count(), 0);

        // forward move respect pin
        let c3_pinned_fen = "rnbqk1nr/ppp2ppp/4p3/b2NP2Q/3P4/2P2P2/PP4PP/R1B1KBNR w KQkq -";
        let c3_pinned_pos = Position::from_fen(c3_pinned_fen).unwrap();
        let c3_moves = &mut MoveList::empty();

        generate_pawn_moves(&c3_pinned_pos, Square::C3, c3_moves);
        assert_eq!(c3_moves.count(), 0);
    }

    #[test]
    fn test_pawn_blocks_check_pawn_start() {
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/Q1P5/8/PP1PPPPP/RNB1KBNR b KQkq -";
        let pos = Position::from_fen(fen).unwrap();

        let c7_moves = &mut MoveList::empty();
        let b7_moves = &mut MoveList::empty();

        generate_pawn_moves(&pos, Square::B7, b7_moves);
        generate_pawn_moves(&pos, Square::C7, c7_moves);

        assert_eq!(c7_moves.count(), 1);
        assert_eq!(b7_moves.count(), 1);

        // TODO: add diagonal test cases
    }

    #[test]
    fn test_pawn_blocks_check_promo() {
        let ok_fen = "8/8/8/8/8/8/5p2/1K1k3R b - -";
        let ok_pos = Position::from_fen(ok_fen).unwrap();

        let ok_moves = &mut MoveList::empty();
        generate_pawn_moves(&ok_pos, Square::F2, ok_moves);

        assert_eq!(ok_moves.count(), 1);

        let not_ok_fen = "8/8/8/8/8/2K5/5p2/R2k4 b - -";
        let not_ok_pos = Position::from_fen(not_ok_fen).unwrap();

        let not_ok_moves = &mut MoveList::empty();
        generate_pawn_moves(&not_ok_pos, Square::F2, not_ok_moves);

        assert_eq!(not_ok_moves.count(), 0);
    }

    #[test]
    fn test_pawn_blocks_check_capture() {
        let non_checking_piece_capture_se_diag_fen = "8/8/8/8/8/2K5/5p2/R2k2N1 b - -";
        let non_checking_piece_capture_sw_diag_fen = "8/8/8/8/8/2K5/5p2/R2kN3 b - -";
        let se_pos = Position::from_fen(non_checking_piece_capture_se_diag_fen).unwrap();
        let sw_pos = Position::from_fen(non_checking_piece_capture_sw_diag_fen).unwrap();

        let se_moves = &mut MoveList::empty();
        let sw_moves = &mut MoveList::empty();
        generate_pawn_moves(&se_pos, Square::F2, se_moves);
        generate_pawn_moves(&sw_pos, Square::F2, sw_moves);

        assert_eq!(se_moves.count(), 0);
        assert_eq!(sw_moves.count(), 0);
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
