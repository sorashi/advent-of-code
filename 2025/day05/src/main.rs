use std::io::stdin;

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
    intervals.sort_by_key(|interval| (*interval.start(), *interval.end()));
    let mut gold = 0;
    let mut last = *intervals[0].start();
    for interval in intervals {
        last = last.max(*interval.start());
        if last <= *interval.end() + 1 {
            gold += interval.end() + 1 - last;
        }
        last = (*interval.end() + 1).max(last);
    }
    println!("silver: {silver}");
    println!("gold: {gold}");
}
