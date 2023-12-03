use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

struct NumberLoc {
    j: usize,
    start: usize,
    end: usize,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct GearLoc {
    j: usize,
    i: usize,
}

#[derive(Debug)]
struct PossibleGear {
    adjacent_numbers: i32,
    gear_ratio: u64,
}

fn is_part(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn increment_possible_gear(
    possible_gears: &mut HashMap<GearLoc, PossibleGear>,
    j: usize,
    i: usize,
    the_number: u64,
) {
    let possible_gear = possible_gears
        .entry(GearLoc { j, i })
        .or_insert(PossibleGear {
            adjacent_numbers: 0,
            gear_ratio: 1,
        });
    possible_gear.adjacent_numbers += 1;
    possible_gear.gear_ratio *= the_number;
}

fn is_part_number(
    inp: &Vec<String>,
    loc: &NumberLoc,
    possible_gears: &mut HashMap<GearLoc, PossibleGear>,
) -> bool {
    let (width, height) = (inp.len(), inp[0].len());
    let from = if loc.start == 0 { 0 } else { loc.start - 1 };
    let to = if loc.end == width { width } else { loc.end + 1 };
    let the_number: u64 = inp[loc.j][loc.start..loc.end].parse().unwrap();
    let mut is_this_part_number = false;
    // check above
    if loc.j > 0 {
        for (i, chr) in inp[loc.j - 1][from..to].chars().enumerate() {
            let i = from + i;
            if is_part(chr) {
                is_this_part_number = true;
            }
            if chr == '*' {
                increment_possible_gear(possible_gears, loc.j - 1, i, the_number);
            }
        }
    }
    // check left right
    let left = inp[loc.j][from..from + 1].chars().next().unwrap();
    let right = inp[loc.j][to - 1..to].chars().next().unwrap();
    if is_part(left) {
        is_this_part_number = true;
    }
    if left == '*' {
        increment_possible_gear(possible_gears, loc.j, from, the_number);
    }
    if is_part(right) {
        is_this_part_number = true;
    }
    if right == '*' {
        increment_possible_gear(possible_gears, loc.j, to - 1, the_number);
    }
    // check below
    if loc.j < height - 1 {
        for (i, chr) in inp[loc.j + 1][from..to].chars().enumerate() {
            let i = from + i;
            if is_part(chr) {
                is_this_part_number = true;
            }
            if chr == '*' {
                increment_possible_gear(possible_gears, loc.j + 1, i, the_number);
            }
        }
    }
    return is_this_part_number;
}

fn main() -> Result<()> {
    let inp = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let number_pattern = Regex::new(r"\d+")?;
    let mut locs = vec![];
    let mut possible_gears = HashMap::new();
    for (i, line) in inp.iter().enumerate() {
        for cap in number_pattern.captures_iter(line) {
            let mat = cap.get(0).unwrap();
            locs.push(NumberLoc {
                j: i,
                start: mat.start(),
                end: mat.end(),
            })
        }
    }
    let mut silver = 0;
    for loc in locs {
        if is_part_number(&inp, &loc, &mut possible_gears) {
            let number: u64 = inp[loc.j][loc.start..loc.end].parse()?;
            silver += number;
        }
    }
    println!("silver: {}", silver);

    let gold: u64 = possible_gears
        .values()
        .filter(|pg| pg.adjacent_numbers == 2)
        .map(|pg| pg.gear_ratio)
        .sum();
    println!("gold: {}", gold);
    Ok(())
}
