use std::{fmt::Debug, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign}};

use crate::board::types::{Square, File, Rank, EnumToArray};
use crate::board::utils::set_bits;

const WHITE_SQUARE: u64 = 0x55AA55AA55AA55AA;
const BLACK_SQUARES: u64 = 0xAA55AA55AA55AA55;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard(pub u64);

impl From<u64> for Bitboard {
    fn from(b: u64) -> Self {
        Self(b)
    }
}

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<Vec<Square>> for Bitboard {
  fn from(v: Vec<Square>) -> Self {
    let mut bb = Self::empty();
    for sq in v {
      bb |= sq.into();
    }
    bb
  }
}

impl Into<Vec<Square>> for Bitboard {
  fn into(self) -> Vec<Square> {
    let set = set_bits(self.into());
    let mut squares = Vec::new();
    for s in set {
        if let Some(sq) = Square::array().get(s) {
            squares.push(*sq)
        }
    }
    squares
  }
}

impl Bitboard {
    fn empty() -> Self {
        Bitboard(0x0)
    }

    fn universe() -> Self {
        Bitboard(!0x0)
    }

    fn pop_count(&self) -> u32 {
      self.0.count_ones()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {

  fn bitand_assign(&mut self, rhs: Self) {
      self.0 &= rhs.0; 
  }

}

impl BitOr for Bitboard {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
      Bitboard(self.0 | rhs.0)
  }
}

impl BitOrAssign for Bitboard {
  fn bitor_assign(&mut self, rhs: Self) {
      self.0 |= rhs.0;
  }

}

impl BitXor for Bitboard {
  type Output = Self;

  fn bitxor(self, rhs: Self) -> Self::Output {
      Bitboard(self.0 ^ rhs.0)
  }

}

impl BitXorAssign for Bitboard {
  fn bitxor_assign(&mut self, rhs: Self) {
      self.0 ^= rhs.0;
  }
}


impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_br = "+---+---+---+---+---+---+---+---+\n";
        f.write_str(line_br)?;
        for rank in Rank::array().iter().rev() {
            f.write_str(format!("{} ", *rank as usize).as_str())?;
            for file in File::array().iter().rev() {
                let sq: Bitboard = Square::new(*file, *rank).into();
                let s = if (*self & sq).0 != 0x0 { "| X " } else { "|   " };
                f.write_str(s)?;

            }
            f.write_str("|\n")?;
            f.write_str(line_br)?;
        }
        f.write_str("    a   b   c   d   e   f   g   h  \n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_u64() {
      let b = 0x15;
      let n = 21;
      assert_eq!(Bitboard::from(b), Bitboard(n));
      assert_eq!(Bitboard::from(n), Bitboard(b));
    }

    #[test]
    fn test_into_u64() {
      let b: u64 = Bitboard(0x15).into();
      assert_eq!(b, 21); 
    }

    #[test]
    fn test_from_vec_square() {
      let sq = vec![Square::A2, Square::F7];
      let a2_bb: Bitboard = Square::A2.into();
      let f7_bb: Bitboard = Square::F7.into();
      assert_eq!(Bitboard::from(sq), a2_bb | f7_bb)
    }

    #[test]
    fn test_into_vec_square() {
      let mut e1 = vec![Square::A1, Square::B1, Square::D1];
      let bb1 = Bitboard(0xb);
      let mut r1: Vec<Square> = bb1.into();
      e1.sort();
      r1.sort();
      assert_eq!(e1, r1);

      let mut e2 = vec![Square::C1, Square::D1, Square::E1];
      let bb2 = Bitboard(0x1c);
      let mut r2: Vec<Square> = bb2.into();
      e2.sort();
      r2.sort();
      assert_eq!(e2, r2)
    }


    #[test]
    fn test_bit_and() {
      let bb = Bitboard(0x15);
      let and = bb & Bitboard(0x1);
      assert_eq!(and, Bitboard(0x1));
      // identity
      assert_eq!(bb & Bitboard(0x15), Bitboard(0x15));
    }

    #[test]
    fn test_bit_and_assign() {
      let mut bb = Bitboard(0b100);
      bb &= Bitboard(0b101);
      assert_eq!(bb, Bitboard(0b100));
    }

    #[test]
    fn test_bit_or() {
      let bb = Bitboard(0x1);
      let or = bb | Bitboard(0xf);
      assert_eq!(or, Bitboard(0xf));
      // identity
      assert_eq!(bb | Bitboard(0x1), Bitboard(0x1));
    }

    #[test]
    fn test_bit_or_assign() {
      let mut bb = Bitboard(0x1);
      bb |= Bitboard(0x1);
      assert_eq!(bb, Bitboard(0x1));
      bb |= Bitboard(0x11);
      assert_eq!(bb, Bitboard(0x11));
    }

    #[test]
    fn test_bit_xor() {
      let bb = Bitboard(0x101);
      let xor = bb ^ Bitboard(0x100);
      let union = bb ^ Bitboard(0x010);
      assert_eq!(xor, Bitboard(0x001));
      assert_eq!(union, Bitboard(0x111));
    }

    #[test]
    fn test_bit_xor_assign() {
      let mut bb = Bitboard(0x10);
      bb ^= Bitboard(0x1);
      assert_eq!(bb, Bitboard(0x11));
      bb ^= Bitboard(0x1);
      assert_eq!(bb, Bitboard(0x10)); 
    }

    #[test]
    fn test_empty() {
      let bb = Bitboard::empty();
      assert_eq!(bb, Bitboard(0));
      assert_eq!(bb, Bitboard(0x0));
    }

    #[test]
    fn test_universe() {
      let bb = Bitboard::universe();
      assert_eq!(bb, Bitboard(u64::MAX));
      assert_eq!(bb, Bitboard(!0x0));
    }

    #[test]
    fn test_pop_count() {
      let mut bb = Bitboard::empty();
      for i in 0..63 {
        assert_eq!(bb.pop_count(), i);
        bb |= Bitboard(0x1 << i);
        assert_eq!(bb.pop_count(), i + 1);
      }
    }

    fn rm_whitespace(s: impl ToString) -> String {
        let mut out = s.to_string();
        out.retain(|c| !c.is_whitespace());
        out
    }

    #[test]
    fn test_debug() {
        let empty_board_fmt = rm_whitespace("
          +---+---+---+---+---+---+---+---+        
        8 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        7 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        6 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        5 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        4 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        3 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        2 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
        1 |   |   |   |   |   |   |   |   |
          +---+---+---+---+---+---+---+---+
            a   b   c   d   e   f   g   h      
        ");
        assert_eq!(rm_whitespace(format!("{:?}", Bitboard::empty())), empty_board_fmt);
        let full_board_fmt = rm_whitespace("
        +---+---+---+---+---+---+---+---+        
      8 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      7 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      6 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      5 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      4 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      3 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      2 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
      1 | X | X | X | X | X | X | X | X |
        +---+---+---+---+---+---+---+---+
          a   b   c   d   e   f   g   h      
      ");
      assert_eq!(rm_whitespace(format!("{:?}", Bitboard::universe())), full_board_fmt);
    }

}