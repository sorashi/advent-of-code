use anyhow::Result;
use regex::Regex;

struct NumberLoc {
    j: usize,
    start: usize,
    end: usize,
}

fn is_part_number(inp: &Vec<String>, loc: &NumberLoc) -> bool {
    let (width, height) = (inp.len(), inp[0].len());
    let from = if loc.start == 0 { 0 } else { loc.start - 1 };
    let to = if loc.end == width {
        width
    } else {
        loc.end + 1
    };
    // check above
    if loc.j > 0 {
        if inp[loc.j - 1][from..to]
            .chars()
            .any(|c| c != '.' && !c.is_digit(10))
        {
            return true;
        }
    }
    // check left right
    let left = inp[loc.j][from..from + 1].chars().next().unwrap();
    let right = inp[loc.j][to - 1..to].chars().next().unwrap();
    if left != '.' && !left.is_digit(10) {
        return true;
    }
    if right != '.' && !right.is_digit(10) {
        return true;
    }
    // check below
    if loc.j < height - 1 {
        if inp[loc.j + 1][from..to].chars().any(|c| c != '.' && !c.is_digit(10)) {
            return true;
        }
    }
    return false;
}

fn main() -> Result<()> {
    let inp = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let width = inp.len();
    let height = inp[0].len();
    let number_pattern = Regex::new(r"\d+")?;
    let mut locs = vec![];
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
        if is_part_number(&inp, &loc) {
            let number: u64 = inp[loc.j][loc.start..loc.end].parse()?;
            silver += number;
        }
    }
    println!("silver: {}", silver);
    Ok(())
}
