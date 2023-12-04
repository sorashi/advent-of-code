use std::{collections::HashSet, io::stdin};

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
        for num in line[pipe+1..].split(' ').filter(|x| !x.is_empty()).map(|x|x.parse::<u64>().unwrap()) {
            if winning.contains(&num) {
                if local_silver == 0 {
                    local_silver = 1;
                }
                else {
                    local_silver *= 2;
                }
            }
        }
        silver += local_silver;
    }
    silver
}

fn main() -> Result<()> {
    let lines = stdin().lines().map(|l|l.unwrap()).collect();
    println!("silver: {}", silver(&lines));
    Ok(())
}
