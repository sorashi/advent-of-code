use aoc_utils::Vector;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn get_count(
    rocks: &HashSet<Vector>,
    size: usize,
    starting_position: Vector,
    steps: usize,
) -> usize {
    let mut possibilities: HashSet<Vector> = HashSet::new();
    possibilities.insert(starting_position);
    let directions = [Vector::LEFT, Vector::RIGHT, Vector::UP, Vector::DOWN];
    for _ in 1..=steps {
        let mut new_possibilities: HashSet<Vector> = HashSet::new();
        for possibility in &possibilities {
            for direction in directions {
                let new_pos = *possibility + direction;
                if new_pos.x >= 0
                    && new_pos.x < size as isize
                    && new_pos.y >= 0
                    && new_pos.y < size as isize
                    && !rocks.contains(&new_pos)
                {
                    new_possibilities.insert(new_pos);
                }
            }
        }
        possibilities = new_possibilities;
    }
    possibilities.len()
}

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

    // the map is a square
    assert_eq!(width, height);
    let size = width;
    // the starting position is in the middle of the map
    assert_eq!(
        starting_position,
        Vector {
            x: size as isize / 2,
            y: size as isize / 2
        }
    );
    let starting_coor = starting_position.x;
    // the map size is odd
    assert_eq!(size % 2, 1);

    println!("silver: {}", get_count(&rocks, size, starting_position, 64));

    const GOLD_STEPS: usize = 26_501_365;

    // the simulation ends at the end of a grid
    assert_eq!(GOLD_STEPS % size, size / 2);

    let width_map_count = GOLD_STEPS / size - 1;

    let odd_maps = ((width_map_count & (!1)) + 1).pow(2);
    let even_maps = ((width_map_count + 1) & (!1)).pow(2);

    let odd_count = get_count(&rocks, size, starting_position, 2 * size + 1);
    let even_count = get_count(&rocks, size, starting_position, 2 * size);

    let mut gold = odd_maps * odd_count + even_maps * even_count;
    // corners
    for spos in [
        Vector {
            x: starting_coor,
            y: size as isize - 1,
        },
        Vector {
            x: size as isize - 1,
            y: starting_coor,
        },
        Vector {
            x: starting_coor,
            y: 0,
        },
        Vector {
            x: 0,
            y: starting_coor,
        },
    ] {
        gold += get_count(&rocks, size, spos, size - 1);
    }

    for x in [0, size as isize - 1] {
        for y in [0, size as isize - 1] {
            // small slice
            gold += (width_map_count + 1) * get_count(&rocks, size, Vector { x, y }, size / 2 - 1);
            // large slice
            gold += width_map_count * get_count(&rocks, size, Vector { x, y }, size * 3 / 2 - 1);
        }
    }

    println!("gold: {}", gold);
}
