use std::{io::stdin, ops::RangeBounds};

fn main() {
    let mut intervals = vec![];
    let mut stock = false;
    let mut silver = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            stock = true;
            continue;
        }
        if !stock {
            let (start, end) = line.split_once('-').unwrap();
            intervals.push(start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap());
            continue;
        }
        let num = line.parse::<u64>().unwrap();
        if intervals.iter().any(|interval| interval.contains(&num)) {
            silver += 1;
        }
    }
    println!("silver: {silver}");
}
