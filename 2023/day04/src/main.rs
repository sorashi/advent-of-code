use std::{collections::HashSet, io::stdin};

fn main() {
    let matching_num_count: Vec<u64> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let colon = line.find(':').unwrap();
            let pipe = line.find('|').unwrap();
            let winning: HashSet<_> = line[colon + 1..pipe]
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            line[pipe + 1..]
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .filter(|x| winning.contains(x))
                .count() as u64
        })
        .collect();

    let silver: u64 = matching_num_count
        .iter()
        .map(|count| if *count == 0 { 0 } else { 1 << (count - 1) })
        .sum();

    let mut winnings = vec![1u64; matching_num_count.len()];
    for i in 0..winnings.len() {
        let w = winnings[i];
        for j in (i + 1)..=(i + matching_num_count[i] as usize) {
            if let Some(x) = winnings.get_mut(j) {
                *x += w;
            }
        }
    }
    let gold: u64 = winnings.iter().sum();
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
