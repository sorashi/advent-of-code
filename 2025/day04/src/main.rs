use std::io::stdin;

use aoc_utils::{TwoDimArray, Vector};

fn main() {
    let mut arr = vec![];
    let mut silver = 0;
    let mut gold = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let mut row = vec![];
        row.extend_from_slice(line.as_bytes());
        arr.push(row);
    }
    let mut arr: TwoDimArray<u8> = arr.into();
    let mut removable = find_removable_rolls(&arr);
    silver = removable.len();
    gold += removable.len();
    while !removable.is_empty() {
        remove_removable_rolls(&mut arr, &removable);
        removable = find_removable_rolls(&arr);
        gold += removable.len();
    }

    println!("silver: {silver}");
    println!("gold: {gold}");
}
fn remove_removable_rolls(arr: &mut TwoDimArray<u8>, removable: &[Vector]) {
    for pos in removable {
        debug_assert!(arr.get_by_vector(pos).unwrap() == &b'@');
        arr.set(pos.x as usize, pos.y as usize, b'.').unwrap();
    }
}
fn find_removable_rolls(arr: &TwoDimArray<u8>) -> Vec<Vector> {
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
    let mut res = vec![];
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
                res.push(pos);
            }
        }
    }
    res
}
