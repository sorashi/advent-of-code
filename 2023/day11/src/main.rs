use std::io::stdin;
use itertools::Itertools;

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

const EXPANSION: N = 999999;

fn main() {
    let mut empty_rows = 0;
    let mut lines = vec![];
    let mut galaxy_map = vec![];
    for (j, line) in stdin().lines().enumerate() {
        let line = line.unwrap();
        galaxy_map.push(vec![None; line.len()]);
        let mut empty = true;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                empty = false;
                let galaxy = Position { x: i, y: j + empty_rows * EXPANSION };
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
                galaxy.x = i + empty_columns * EXPANSION;
            }
        }
        if empty {
            empty_columns += 1;
        }
    }

    let galaxies: Vec<Position> = galaxy_map.into_iter().flatten().flatten().collect();
    let silver: N = galaxies.iter().combinations(2).map(|c| c[0].distance(c[1])).sum();
    println!("silver: {}", silver);
}
