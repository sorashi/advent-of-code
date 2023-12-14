use std::collections::HashMap;
use std::io::{stdin, Read};

fn get_total_load<S: AsRef<[u8]>>(map: &[S]) -> usize {
    let width = map[0].as_ref().len();
    let height = map.len();
    let mut load = 0;
    for x in 0..width {
        for y in 0..height {
            if map[y].as_ref()[x] == b'O' {
                load += height - y;
            }
        }
    }
    load
}

fn tilt<G, S>(map: &mut Vec<Vec<u8>>, width: usize, height: usize, getter: G, setter: S)
where
    G: Fn(&Vec<Vec<u8>>, usize, usize) -> u8,
    S: Fn(&mut Vec<Vec<u8>>, usize, usize, u8),
{
    for x in 0..width {
        let mut rounded_rocks = 0;
        let mut from = 0;
        for y in 0..height {
            if getter(map, x, y) == b'O' {
                rounded_rocks += 1;
                setter(map, x, y, b'.');
            } else if getter(map, x, y) == b'#' {
                for new_y in from..(from + rounded_rocks) {
                    setter(map, x, new_y, b'O');
                }
                from = y + 1;
                setter(map, x, y, b'#');
                rounded_rocks = 0;
            }
        }
        for new_y in from..(from + rounded_rocks) {
            setter(map, x, new_y, b'O');
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<u8>>) {
    let width = map[0].len();
    let height = map.len();
    tilt(
        map,
        height,
        width,
        |m, x, y| m[x][width - 1 - y],
        |m, x, y, v| m[x][width - 1 - y] = v,
    );
}

fn tilt_west(map: &mut Vec<Vec<u8>>) {
    let width = map[0].len();
    let height = map.len();
    tilt(
        map,
        height,
        width,
        |m, x, y| m[x][y],
        |m, x, y, v| m[x][y] = v,
    );
}

fn tilt_south(map: &mut Vec<Vec<u8>>) {
    let width = map[0].len();
    let height = map.len();
    tilt(
        map,
        width,
        height,
        |m, x, y| m[height - 1 - y][x],
        |m, x, y, v| m[height - 1 - y][x] = v,
    );
}

fn tilt_north(map: &mut Vec<Vec<u8>>) {
    let width = map[0].len();
    let height = map.len();
    tilt(
        map,
        width,
        height,
        |m, x, y| m[y][x],
        |m, x, y, v| m[y][x] = v,
    );
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut map: Vec<Vec<_>> = input
        .split_terminator('\n')
        .map(|x| x.trim().as_bytes().to_vec())
        .collect();
    tilt_north(&mut map);
    let silver = get_total_load(&map);
    println!("silver: {}", silver);

    // finish the first iteration
    tilt_west(&mut map);
    tilt_south(&mut map);
    tilt_east(&mut map);

    let mut hashmap = HashMap::new();
    hashmap.insert(map.clone(), 1);
    let mut remaining_iterations = 0;
    const ITERS: usize = 1_000_000_000;
    for i in 2..=ITERS {
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);
        let entry = hashmap.get(&map);
        if let Some(iteration) = entry {
            remaining_iterations = (ITERS - i) % (i - *iteration);
            break;
        }
        hashmap.insert(map.clone(), i);
    }
    for _ in 1..=remaining_iterations {
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);
    }
    let gold = get_total_load(&map);
    println!("gold: {}", gold);
}
