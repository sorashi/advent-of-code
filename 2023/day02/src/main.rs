use anyhow::Result;
use regex::Regex;

fn possible_hand(red: u64, green: u64, blue: u64) -> bool {
    red <= 12 && green <= 13 && blue <= 14
}

fn main() -> Result<()> {
    let mut possible_games = 0u64;
    let mut cube_powers = 0u64;

    let re = Regex::new(r"(?P<number>\d+) (?P<color>red|green|blue)")?;
    let mut index = 1;
    for line in std::io::stdin().lines() {
        let line = line?;
        let start = line.find(':').unwrap();
        let mut possible = true;
        let mut max_colors = [0u64; 3];
        for hand in line[start + 1..].split(';') {
            let mut colors = [0u64; 3];
            for cap in re.captures_iter(hand) {
                let count: u64 = cap.name("number").unwrap().as_str().parse()?;
                match cap.name("color").unwrap().as_str() {
                    "red" => colors[0] += count,
                    "green" => colors[1] += count,
                    "blue" => colors[2] += count,
                    _ => unreachable!(),
                }
            }
            if !possible_hand(colors[0], colors[1], colors[2]) {
                possible = false;
            }
            for (color, max_color) in colors.iter().zip(max_colors.iter_mut()) {
                if *color > *max_color {
                    *max_color = *color;
                }
            }
        }
        if possible {
            possible_games += index;
        }
        cube_powers += max_colors[0] * max_colors[1] * max_colors[2];
        index += 1;
    }
    println!("Silver: {}", possible_games);
    println!("Gold: {}", cube_powers);
    Ok(())
}
