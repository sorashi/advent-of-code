use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use anyhow::Result;

fn silver(lines: &Vec<String>) -> u64 {
    let mut silver = 0u64;
    for line in lines {
        let colon = line.find(':').unwrap();
        let pipe = line.find('|').unwrap();
        let winning: HashSet<_> = line[colon + 1..pipe]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let mut local_silver = 0u64;
        for num in line[pipe + 1..]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
        {
            if winning.contains(&num) {
                if local_silver == 0 {
                    local_silver = 1;
                } else {
                    local_silver *= 2;
                }
            }
        }
        silver += local_silver;
    }
    silver
}

fn gold(lines: &Vec<String>) -> u64 {
    let mut matching_num_count = vec![0u64; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let colon = line.find(':').unwrap();
        let pipe = line.find('|').unwrap();
        let winning: HashSet<_> = line[colon + 1..pipe]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        matching_num_count[idx] = line[pipe + 1..]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .filter(|x| winning.contains(x))
            .count() as u64;
    }
    let mut winnings = vec![1u64; lines.len()];
    for i in 0..winnings.len() {
        let w = winnings[i];
        for j in (i + 1)..=(i + matching_num_count[i] as usize) {
            match winnings.get_mut(j) {
                Some(x) => *x += w,
                _ => {}
            }
        }
    }
    return winnings.iter().sum();
}

fn main() -> Result<()> {
    let lines = stdin().lines().map(|l| l.unwrap()).collect();
    println!("silver: {}", silver(&lines));
    println!("gold: {}", gold(&lines));
    Ok(())
}
