use std::io::stdin;

type N = u64;
fn first_max(bts: &[u8]) -> (usize, u8) {
    let mut curr = u8::MIN;
    let mut idx = 0;
    for i in 0..bts.len() {
        let v = unsafe { *bts.get_unchecked(i) };
        if v > curr {
            idx = i;
            curr = v;
        }
    }
    (idx, curr)
}
fn get_joltage(mut bts: &[u8], mut count: usize) -> N {
    let mut result = 0;
    while count > 0 {
        let (idx, v) = first_max(&bts[0..bts.len() + 1 - count]);
        debug_assert!(v.is_ascii_digit());
        result = 10 * result + (v - b'0') as N;
        bts = &bts[idx + 1..];
        count -= 1;
    }
    result
}
fn main() {
    let mut silver = 0;
    let mut gold = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let bts = line.as_bytes();
        silver += get_joltage(bts, 2);
        gold += get_joltage(bts, 12);
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_joltage() {
        assert_eq!(get_joltage(b"119198", 2), 99);
        assert_eq!(get_joltage(b"119119", 3), 919);
    }
}
