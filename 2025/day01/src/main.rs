use std::io::stdin;

fn main() {
    let mut position = 50;
    let mut silver = 0;
    let mut gold = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let amount = line[1..].parse::<i32>().unwrap();
        let prev_pos = position;
        match &line[0..1] {
            "L" => position -= amount,
            "R" => position += amount,
            _ => unreachable!(),
        }
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
