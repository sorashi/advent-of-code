use aoc_utils::Vector;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut rocks: HashSet<Vector> = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    let mut starting_position = Vector::ZERO;
    for (y, line) in input.split_terminator('\n').enumerate() {
        width = line.len();
        height += 1;
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => {
                    rocks.insert(Vector {
                        x: x as isize,
                        y: y as isize,
                    });
                }
                b'S' => {
                    starting_position = Vector {
                        x: x as isize,
                        y: y as isize,
                    }
                }
                _ => {}
            }
        }
    }

    let mut possibilities: HashSet<Vector> = HashSet::new();
    possibilities.insert(starting_position);
    let directions = [Vector::LEFT, Vector::RIGHT, Vector::UP, Vector::DOWN];
    for _ in 1..=64 {
        let mut new_possibilities: HashSet<Vector> = HashSet::new();
        for possibility in &possibilities {
            for direction in directions {
                let new_pos = *possibility + direction;
                if new_pos.x >= 0
                    && new_pos.y < width as isize
                    && new_pos.y >= 0
                    && new_pos.y < height
                    && !rocks.contains(&new_pos)
                {
                    new_possibilities.insert(new_pos);
                }
            }
        }
        possibilities = new_possibilities;
    }
    println!("silver: {}", possibilities.len());
}
