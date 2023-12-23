use core::panic;
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

use aoc_utils::Vector;

static DIRECTIONS: [Vector; 4] = [Vector::LEFT, Vector::RIGHT, Vector::UP, Vector::DOWN];

type Graph = HashMap<Vector, HashMap<Vector, usize>>;
fn get_max_path(
    point: Vector,
    graph: &Graph,
    visited: &mut HashSet<Vector>,
    target: Vector,
) -> usize {
    if point == target {
        return 0;
    }
    let mut m = 0;

    visited.insert(point);
    for (npos, n) in &graph[&point] {
        m = m.max(get_max_path(*npos, graph, visited, target) + n)
    }
    m
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut map: Vec<&[u8]> = vec![];
    for line in input.split_terminator('\n') {
        map.push(line.as_bytes());
    }
    let width = map[0].len() as isize;
    let height = map.len() as isize;
    let start = Vector { x: 1, y: 0 };
    let target = Vector {
        x: width - 2,
        y: height - 1,
    };

    let mut crossroads = vec![];
    for y in 0..height {
        for x in 0..width {
            let pos = Vector { x, y };
            if *pos.get(&map).unwrap() == b'#' {
                continue;
            }
            let n = DIRECTIONS
                .map(|d| d + pos)
                .map(|npos| npos.get(&map))
                .iter()
                .filter(|b| matches!(b, Some(b) if **b != b'#'))
                .count();
            if n > 2 {
                crossroads.push(pos);
            }
        }
    }

    let mut pois = crossroads;
    pois.push(start);
    pois.push(target);

    let mut graph: Graph = pois.iter().map(|poi| (*poi, HashMap::new())).collect();
    for poi in &pois {
        let mut stack = vec![(0, *poi)];
        let mut visited = HashSet::new();
        visited.insert(*poi);
        while let Some((n, current)) = stack.pop() {
            if n != 0 && pois.contains(&current) {
                graph.get_mut(poi).unwrap().insert(current, n);
                continue;
            }
            let dirs: &[Vector] = match current.get(&map).unwrap() {
                b'.' => &DIRECTIONS,
                b'>' => &[Vector::RIGHT],
                b'<' => &[Vector::LEFT],
                b'^' => &[Vector::UP],
                b'v' => &[Vector::DOWN],
                _ => panic!(),
            };
            for dir in dirs {
                let pos = current + *dir;
                if let Some(c) = pos.get(&map) {
                    if *c != b'#' && !visited.contains(&pos) {
                        stack.push((n + 1, pos));
                        visited.insert(pos);
                    }
                }
            }
        }
    }

    println!(
        "silver: {}",
        get_max_path(start, &graph, &mut HashSet::new(), target)
    );
}
