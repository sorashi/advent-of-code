use std::io::stdin;

type N = u128;

fn main() {
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    let mut silver = 0;
    for id_pair in inp.trim().split(',') {
        let (start, end) = id_pair.split_once('-').unwrap();
        let mut current = start.split_at(start.len() / 2).0.parse::<N>().unwrap_or(1);
        let start = start.parse::<N>().unwrap();
        let end = end.parse::<N>().unwrap();
        while repeat_num(current) < start {
            current += 1;
        }
        while repeat_num(current) <= end {
            silver += repeat_num(current);
            current += 1;
        }
    }
    println!("silver: {silver}");
}

fn repeat_num(num: N) -> N {
    let digits = num.ilog10() as N;
    let hundreds = 10u128.pow(digits as u32 + 1);
    num * hundreds + num
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_repeat_num() {
        assert_eq!(repeat_num(12), 1212);
        assert_eq!(repeat_num(5), 55);
        assert_eq!(repeat_num(123), 123123);
        assert_eq!(repeat_num(10), 1010);
    }
}
