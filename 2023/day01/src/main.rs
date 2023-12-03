use aho_corasick::{AhoCorasick, Match};
use anyhow::Result;

const PATTERNS: &[&str; 18] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn first_last<I: Iterator>(mut iter: I) -> Option<(I::Item, I::Item)>
where
    I::Item: Clone,
{
    if let Some(first) = iter.next() {
        let last = iter.last().unwrap_or(first.clone());
        return Some((first, last));
    }
    return None;
}

fn get_num<P>(ac: &AhoCorasick, haystack: &str, predicate: P) -> usize
where
    P: FnMut(&Match) -> bool,
{
    let (first, last) = first_last(ac.find_overlapping_iter(haystack).filter(predicate)).unwrap();
    let first = (first.pattern().as_usize() % 9) + 1;
    let last = (last.pattern().as_usize() % 9) + 1;
    first * 10 + last
}

fn main() -> Result<()> {
    let ac = AhoCorasick::new(PATTERNS)?;
    let mut gold = 0usize;
    let mut silver = 0usize;
    for line in std::io::stdin().lines() {
        let line = line?;
        gold += get_num(&ac, &line, |_| true);
        silver += get_num(&ac, &line, |m| m.pattern().as_usize() <= 8);
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
    Ok(())
}
