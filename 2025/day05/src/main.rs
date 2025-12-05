use std::{io::stdin, ops::RangeInclusive};

/// Merges intervals such that `intervals` contains only non-overlapping intervals sorted by start.
/// Assumes `intervals` is sorted by start value, then by end value.
fn normalize(intervals: &mut Vec<RangeInclusive<u64>>) {
    let mut result = vec![];
    let mut start = *intervals[0].start();
    let mut end = *intervals[0].end();
    for interval in intervals.iter() {
        if end < *interval.start() {
            result.push(start..=end);
            start = *interval.start();
        }
        end = end.max(*interval.end());
    }
    result.push(start..=end);
    std::mem::swap(intervals, &mut result);
}

fn main() {
    let mut intervals = vec![];
    let mut stock = false;
    let mut silver = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            stock = true;
            println!("intervals done {}, now stock", intervals.len());
            intervals.sort_unstable_by_key(|interval: &RangeInclusive<u64>| {
                (*interval.start(), *interval.end())
            });
            normalize(&mut intervals);
            continue;
        }
        if !stock {
            let (start, end) = line.split_once('-').unwrap();
            intervals.push(start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap());
            continue;
        }
        let num = line.parse::<u64>().unwrap();
        if intervals
            .binary_search_by(|interval| {
                if num < *interval.start() {
                    std::cmp::Ordering::Greater
                } else if num > *interval.end() {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
        {
            silver += 1;
        }
    }

    // intervals are non-overlapping, so just sum their lengths
    let gold = intervals
        .iter()
        .map(|interval| interval.end() - interval.start() + 1)
        .sum::<u64>();
    println!("silver: {silver}");
    println!("gold: {gold}");
}
