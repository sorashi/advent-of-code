use aho_corasick::AhoCorasick;
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

fn main() -> Result<()> {
    let ac = AhoCorasick::new(PATTERNS)?;
    let mut sum = 0usize;
    for line in std::io::stdin().lines() {
        let l = line?;
        let (first, last) = first_last(ac.find_overlapping_iter(&l)).unwrap();
        let first = (first.pattern().as_usize() % 9) + 1;
        let last = (last.pattern().as_usize() % 9) + 1;
        let num: usize = first * 10 + last;
        sum += num;
    }
    println!("{}", sum);
    Ok(())
}
