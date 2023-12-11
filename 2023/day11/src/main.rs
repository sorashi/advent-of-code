use std::cmp::Ordering;
use std::io::stdin;
use itertools::Itertools;

type N = usize;

#[derive(Debug)]
struct Position {
    id: N,
    x: N,
    y: N,
}

impl Position {
    fn distance(&self, other: &Position) -> N {
        other.y.abs_diff(self.y) + other.x.abs_diff(self.x)
    }
}

fn main() {
    let mut galaxies = vec![];
    let mut id: N = 1;
    let mut empty_rows = 0;
    let mut lines = vec![];
    for (j, line) in stdin().lines().enumerate() {
        let line = line.unwrap();
        let mut empty = true;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                empty = false;
                galaxies.push(Position { id, x: i, y: j + empty_rows });
                id += 1;
            }
        }
        if empty {
            empty_rows += 1;
        }
        lines.push(line);
    }
    let mut empty_columns = 0;
    for i in (0..lines[0].len()).rev() {
        let mut empty = true;
        for j in 0..lines.len() {
            if lines[j].as_bytes()[i] == '#' as u8 {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns += 1;
            galaxies.iter_mut().filter(|g| g.x > i).for_each(|f| f.x += 1);
        }
    }

    let width = galaxies.iter().map(|g| g.x).max().unwrap();
    let height = galaxies.iter().map(|g| g.y).max().unwrap();
    id = 0;
    for j in 0..=height {
        for i in 0..=width {
            eprint!("{}", if galaxies.iter().any(|g| g.x == i && g.y == j) {
                id += 1;
                id.to_string()
            } else { ".".to_string() });
        }
        eprintln!();
    }
    galaxies.iter().combinations(2).for_each(|c| {
        eprintln!("galaxies {} and {} distance {}", c[0].id, c[1].id, c[0].distance(c[1]));
    });
    let silver: N = galaxies.iter().combinations(2).map(|c| c[0].distance(c[1])).sum();
    println!("silver: {}", silver);
}
