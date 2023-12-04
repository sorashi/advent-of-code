use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let mut silver = 0u64;
    for line in std::io::stdin().lines() {
        let line = line?;
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
    println!("silver: {}", silver);
    Ok(())
}
