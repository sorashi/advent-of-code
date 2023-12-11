use itertools::Itertools;
use std::io::{stdin, Read};

type N = usize;

#[derive(Debug, Clone)]
struct Position {
    x: N,
    y: N,
}

impl Position {
    fn distance(&self, other: &Position) -> N {
        other.y.abs_diff(self.y) + other.x.abs_diff(self.x)
    }
}

fn silver(input: &str) -> N {
    solve(input, 1)
}

fn gold(input: &str) -> N {
    solve(input, 999999)
}

fn solve(input: &str, expansion: N) -> N {
    let mut empty_rows = 0;
    let mut lines = vec![];
    let mut galaxy_map = vec![];
    for (j, line) in input.split_terminator('\n').enumerate() {
        galaxy_map.push(vec![None; line.len()]);
        let mut empty = true;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                empty = false;
                let galaxy = Position {
                    x: i,
                    y: j + empty_rows * expansion,
                };
                galaxy_map[j][i] = Some(galaxy);
            }
        }
        if empty {
            empty_rows += 1;
        }
        lines.push(line);
    }
    let mut empty_columns = 0;
    for i in 0..galaxy_map[0].len() {
        let mut empty = true;
        for j in 0..galaxy_map.len() {
            if let Some(galaxy) = &mut galaxy_map[j][i] {
                empty = false;
                galaxy.x = i + empty_columns * expansion;
            }
        }
        if empty {
            empty_columns += 1;
        }
    }

    let galaxies: Vec<Position> = galaxy_map.into_iter().flatten().flatten().collect();
    galaxies
        .iter()
        .combinations(2)
        .map(|c| c[0].distance(c[1]))
        .sum()
}

fn main() {
    let mut inp = String::new();
    stdin().read_to_string(&mut inp).unwrap();
    println!("silver: {}", silver(&inp));
    println!("gold: {}", gold(&inp));
}
