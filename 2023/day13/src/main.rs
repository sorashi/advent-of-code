use std::cmp::min;
use std::io::{stdin, Read};

fn check_for_reflections_vertically(
    pattern: &[&[u8]],
    width: usize,
    height: usize,
    accessor: fn(p: &[&[u8]], x: usize, y: usize) -> u8,
) -> (Vec<usize>, Vec<usize>) {
    let mut vertical_reflection_lines_silver = vec![];
    let mut vertical_reflection_lines_gold = vec![];
    for possible_reflection_line in 0..width - 1 {
        let possible_reflection_width = min(
            width - possible_reflection_line - 1,
            possible_reflection_line + 1,
        );
        let mut smudges = 0;
        for x in 0..possible_reflection_width {
            for y in 0..height {
                if accessor(pattern, possible_reflection_line - x, y)
                    != accessor(pattern, possible_reflection_line + x + 1, y)
                {
                    smudges += 1;
                }
            }
        }
        if possible_reflection_width > 0 {
            if smudges == 0 {
                vertical_reflection_lines_silver.push(possible_reflection_line);
            } else if smudges == 1 {
                vertical_reflection_lines_gold.push(possible_reflection_line);
            }
        }
    }
    (
        vertical_reflection_lines_silver,
        vertical_reflection_lines_gold,
    )
}

fn get_pattern_reflections(
    pattern: &[&[u8]],
) -> ((Vec<usize>, Vec<usize>), (Vec<usize>, Vec<usize>)) {
    let width = pattern[0].len();
    let height = pattern.len();
    let vertical_reflection_lines =
        check_for_reflections_vertically(pattern, width, height, |p, x, y| p[y][x]);
    let horizontal_reflection_lines =
        check_for_reflections_vertically(pattern, height, width, |p, x, y| p[x][y]);
    (vertical_reflection_lines, horizontal_reflection_lines)
}

fn compute_answer(vertical: &[usize], horizontal: &[usize]) -> usize {
    vertical.iter().map(|x| x + 1).sum::<usize>()
        + horizontal.iter().map(|x| 100 * (x + 1)).sum::<usize>()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let patterns: Vec<_> = input.split_terminator("\n\n").collect();
    let mut silver = 0usize;
    let mut gold = 0usize;
    for pattern in patterns {
        let pattern: Vec<_> = pattern
            .split_terminator('\n')
            .map(|x| x.trim().as_bytes())
            .collect();
        let (vertical, horizontal) = get_pattern_reflections(&pattern);
        let (vertical_silver, vertical_gold) = vertical;
        let (horizontal_silver, horizontal_gold) = horizontal;
        silver += compute_answer(&vertical_silver, &horizontal_silver);
        gold += compute_answer(&vertical_gold, &horizontal_gold);
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
