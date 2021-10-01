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
