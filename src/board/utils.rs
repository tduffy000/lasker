use crate::board::{
    bitboard::Bitboard,
    board::Board,
    r#move::Move,
    types::{Color, Direction, Square},
};

use super::r#move::MoveList;

// can use Kernighan's algo here
pub fn set_bits(b: u64) -> Vec<usize> {
    let mut v = Vec::new();
    for sh in 0..64 {
        if b & (0x1 << sh) != 0x0 {
            v.push(sh)
        }
    }
    v
}

pub fn recur_move_search(
    board: &Board,
    color: Color,
    dirs: &Vec<Direction>,
    moves: &mut MoveList,
    sq: Square,
    depth: i8,
) -> () {
    let mut to_search = vec![];
    for dir in dirs {
        let mailbox_no = sq + (*dir as i8 * depth);
        if mailbox_no >= 0 {
            let other_sq = Square::from_mailbox_no(mailbox_no);
            if !board.sq_taken_by_color(other_sq, color) {
                let captured = board.piece(&other_sq);
                moves.push(Move::new(sq, other_sq, captured, None, false, false, false));
                if captured.is_none() {
                    to_search.push(*dir)
                }
            }
        }
    }
    if !to_search.is_empty() {
        return recur_move_search(board, color, &to_search, moves, sq, depth + 1);
    }
}

pub fn recur_attack_sq_search(
    board: &Board,
    dirs: Vec<Direction>,
    sq: Square,
    depth: i8,
    attack_bb: Bitboard,
) -> bool {
    let mut to_search = vec![];
    for dir in dirs {
        let mailbox_no = sq + (dir as i8 * depth);
        if mailbox_no >= 0 {
            let other_sq = Square::from_mailbox_no(mailbox_no);
            let other_sq_bb: Bitboard = other_sq.into();
            if (other_sq_bb & attack_bb).0 != 0x0 {
                return true;
            } else if !board.sq_taken(other_sq) {
                to_search.push(dir);
            }
        }
    }
    if !to_search.is_empty() {
        return recur_attack_sq_search(board, to_search, sq, depth + 1, attack_bb);
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_bits() {
        assert_eq!(set_bits(0x0), Vec::<usize>::new());
        // 28 =  + 2 ^ 2 (4) + 3 ^ 2 (8) + 4 ^ 2 (16)
        let mut r1 = set_bits(0x1c);
        let mut e1 = vec![2, 3, 4];
        r1.sort();
        e1.sort();
        assert_eq!(r1, e1);
    }
}
