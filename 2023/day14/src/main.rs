use std::io::{Read, stdin};

fn print_map<S: AsRef<[u8]>>(map: &[S]) {
    for y in 0..map.len() {
        for x in 0..map[0].as_ref().len() {
            eprint!("{}", map[y].as_ref()[x] as char);
        }
        eprintln!();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let map: Vec<_> = input.split_terminator('\n').map(|x| x.trim().as_bytes()).collect();
    let width = map[0].len();
    let height = map.len();
    let mut new_map = vec![vec![b'.'; width]; height];
    let mut silver = 0;
    for x in 0..width {
        let mut rounded_rocks = 0;
        let mut from = 0;
        for y in 0..height {
            if map[y][x] == b'O' {
                rounded_rocks += 1;
            } else if map[y][x] == b'#' {
                for new_y in from..(from + rounded_rocks) {
                    new_map[new_y][x] = b'O';
                    silver += height - new_y;
                }
                from = y + 1;
                new_map[y][x] = b'#';
                rounded_rocks = 0;
            }
        }
        for new_y in from..(from + rounded_rocks) {
            new_map[new_y][x] = b'O';
            silver += height - new_y;
        }
    }
    print_map(&new_map);
    println!("silver: {}", silver);
}