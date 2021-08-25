use std::{fmt::Debug, ops::{BitAnd, BitOr}};

use super::types::{Square, File, Rank};

const BLACK_SQUARES: u64 = 0xAA55AA55AA55AA55;

#[derive(Clone, Copy)]
pub struct Bitboard(pub u64);

impl From<u64> for Bitboard {
    fn from(b: u64) -> Self {
        Bitboard(b)
    }
}

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}

impl Bitboard {
    fn empty() -> Self {
        Bitboard(0x0)
    }

    fn full() -> Self {
        Bitboard(!0x0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_br = "+---+---+---+---+---+---+---+---+\n";
        f.write_str(line_br)?;
        for rank in Rank::get_array().iter().rev() {
            f.write_str(format!("{} ", *rank as usize).as_str())?;
            for file in File::get_array().iter().rev() {
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
      assert_eq!(rm_whitespace(format!("{:?}", Bitboard::full())), full_board_fmt);
    }

}