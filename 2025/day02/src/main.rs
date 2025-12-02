use std::{collections::HashSet, io::stdin};

type N = u128;

fn main() {
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    let mut silver = 0;
    let mut gold = 0;
    for id_pair in inp.trim().split(',') {
        let (start, end) = id_pair.split_once('-').unwrap();
        silver += silver_one(start, end);
        gold += gold_one(start, end);
    }
    println!("silver: {silver}");
    println!("gold: {gold}");
}

fn silver_one(start: &str, end: &str) -> N {
    let mut silver = 0;
    let mut current = start.split_at(start.len() / 2).0.parse::<N>().unwrap_or(1);
    let start = start.parse::<N>().unwrap();
    let end = end.parse::<N>().unwrap();
    while repeat_num(current, 2) < start {
        current += 1;
    }
    while repeat_num(current, 2) <= end {
        silver += repeat_num(current, 2);
        current += 1;
    }
    silver
}
fn gold_one(start: &str, end: &str) -> N {
    let mut gold = 0;
    let start = start.parse::<N>().unwrap();
    let end = end.parse::<N>().unwrap();
    let mut current = 1;
    let mut added = HashSet::<N>::new();
    while repeat_num(current, 2) <= end {
        for i in 2.. {
            if repeat_num(current, i) > end {
                break;
            }
            if repeat_num(current, i) >= start
                && repeat_num(current, i) <= end
                && !added.contains(&repeat_num(current, i))
            {
                gold += repeat_num(current, i);
                added.insert(repeat_num(current, i));
            }
        }
        current += 1;
    }
    gold
}

fn repeat_num(num: N, times: u32) -> N {
    let digits = num.ilog10() as N;
    let hundreds = 10u128.pow(digits as u32 + 1);
    let mut result = num;
    for _ in 1..times {
        result = result * hundreds + num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_repeat_num() {
        assert_eq!(repeat_num(12, 2), 1212);
        assert_eq!(repeat_num(5, 2), 55);
        assert_eq!(repeat_num(123, 2), 123123);
        assert_eq!(repeat_num(10, 2), 1010);
        assert_eq!(repeat_num(1, 7), 1111111);
        assert_eq!(repeat_num(123, 3), 123123123);
        assert_eq!(repeat_num(12, 5), 1212121212);
        assert_eq!(repeat_num(12, 1), 12);
    }
}
