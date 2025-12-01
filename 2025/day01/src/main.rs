use std::io::stdin;

fn main() {
    let mut position = 50;
    let mut silver = 0;
    let mut gold = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let mut amount = line[1..].parse::<i32>().unwrap();
        match &line[0..1] {
            "L" => amount = -amount,
            "R" => (),
            _ => unreachable!(),
        }
        let prev_pos = position;
        position += amount;
        if prev_pos > 0 && position < 0 || prev_pos < 0 && position > 0 {
            gold += 1;
        }
        gold += position.abs() / 100;
        if position % 100 == 0 && position != 0 {
            gold -= 1;
        }

        position = position.rem_euclid(100);
        if position == 0 {
            silver += 1;
        }
    }
    gold += silver;
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
