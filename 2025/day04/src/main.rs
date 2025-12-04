use std::io::stdin;

use aoc_utils::{TwoDimArray, Vector};

fn main() {
    let mut arr = vec![];
    let mut gold = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        arr.push(line.into_bytes());
    }
    let mut arr: TwoDimArray<u8> = arr.into();
    let silver = remove_removable_rolls(&mut arr, false);
    let mut cnt = remove_removable_rolls(&mut arr, true);
    while cnt > 0 {
        gold += cnt;
        cnt = remove_removable_rolls(&mut arr, true);
    }

    println!("silver: {silver}");
    println!("gold: {gold}");
}
fn remove_removable_rolls(arr: &mut TwoDimArray<u8>, remove: bool) -> usize {
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
    let mut res = 0;
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
                if remove {
                    arr.set(pos.x as usize, pos.y as usize, b'.');
                }
                res += 1;
            }
        }
    }
    res
}
