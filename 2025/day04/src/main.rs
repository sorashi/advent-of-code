use std::io::{Read, stdin};

use aoc_utils::{TwoDimArray, Vector};

fn main() {
    let mut arr = vec![];
    let mut silver = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let mut row = vec![];
        row.extend_from_slice(line.as_bytes());
        arr.push(row);
    }
    let arr: TwoDimArray<u8> = arr.into();
    let dirs = [
        Vector::LEFT,
        Vector::RIGHT,
        Vector::UP,
        Vector::DOWN,
        Vector::NW,
        Vector::NE,
        Vector::SW,
        Vector::SE,
    ];
    for y in 0..arr.height() {
        for x in 0..arr.width() {
            let pos = Vector {
                x: x as isize,
                y: y as isize,
            };
            if arr.get_by_vector(&pos).unwrap() != &b'@' {
                continue;
            }
            let cnt = dirs
                .iter()
                .filter(|dir| {
                    arr.get_by_vector(&(**dir + pos))
                        .map(|x| *x == b'@')
                        .unwrap_or(false)
                })
                .count();
            if cnt < 4 {
                silver += 1;
            }
        }
    }
    println!("silver: {silver}");
}
