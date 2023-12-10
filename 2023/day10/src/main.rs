use std::collections::{HashSet, VecDeque};
use std::convert::Into;
use std::io::{Read, stdin};
use std::ops::Add;

const VERTICAL: u8 = '|' as u8;
const HORIZONTAL: u8 = '-' as u8;
const NORTH_EAST: u8 = 'L' as u8;
const NORTH_WEST: u8 = 'J' as u8;
const SOUTH_WEST: u8 = '7' as u8;
const SOUTH_EAST: u8 = 'F' as u8;
const GROUND: u8 = '.' as u8;
const STARTING_POSITION: u8 = 'S' as u8;

type N = isize;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: N,
    y: N,
}

impl Position {
    const LEFT: Position = Position { x: -1, y: 0 };
    const RIGHT: Position = Position { x: 1, y: 0 };
    const UP: Position = Position { x: 0, y: -1 };
    const DOWN: Position = Position { x: 0, y: 1 };
    fn access<T, U>(&self, map: &[U]) -> Option<T> where T: Copy, U: AsRef<[T]> {
        map.get(self.y as usize).and_then(|row| row.as_ref().get(self.x as usize).copied())
    }
    fn access_mut<'a, T>(&self, map: &'a mut Vec<Vec<T>>) -> Option<&'a mut T> where T: Copy {
        map.get_mut(self.y as usize).and_then(|row| row.get_mut(self.x as usize))
    }
}

impl From<(N, N)> for Position {
    fn from((x, y): (N, N)) -> Self {
        Position {
            x,
            y,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

fn get_starting_position(pipe_map: &[&[u8]]) -> Option<Position> {
    for j in 0..pipe_map.len() {
        let i = pipe_map[j].iter().position(|x| *x == STARTING_POSITION);
        if let Some(i) = i {
            return Some((i, j).into());
        }
    }
    None
}

fn main() {
    let mut inp = String::new();
    stdin().read_to_string(&mut inp).unwrap();
    let mut pipe_map = vec![];
    for line in inp.split_terminator('\n') {
        pipe_map.push(line.as_bytes());
    }
    let start = get_starting_position(&pipe_map).unwrap();

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut stack = Vec::new();
    stack.push(start);
    let mut steps_map: Vec<Vec<isize>> = vec![vec![-1; pipe_map[0].len()]; pipe_map.len()];
    *start.access_mut(&mut steps_map).unwrap() = 0;
    let current = stack.pop().unwrap();

    let left = current + Position::LEFT;
    match left.access(&pipe_map) {
        Some(HORIZONTAL) | Some(NORTH_EAST) | Some(SOUTH_EAST) => {
            if !visited.contains(&left) {
                *left.access_mut(&mut steps_map).unwrap() = current.access(&steps_map).unwrap() + 1;
                visited.insert(left);
                stack.push(left)
            }
        }
        _ => {}
    }
    let right = current + Position::RIGHT;
    match right.access(&pipe_map) {
        Some(HORIZONTAL) | Some(NORTH_WEST) | Some(SOUTH_WEST) => {
            if !visited.contains(&right) {
                *right.access_mut(&mut steps_map).unwrap() = current.access(&steps_map).unwrap() + 1;
                visited.insert(right);
                stack.push(right)
            }
        }
        _ => {}
    }
    let up = current + Position::UP;
    match up.access(&pipe_map) {
        Some(VERTICAL) | Some(NORTH_WEST) | Some(NORTH_EAST) => {
            if !visited.contains(&up) {
                *up.access_mut(&mut steps_map).unwrap() = current.access(&steps_map).unwrap() + 1;
                visited.insert(up);
                stack.push(up)
            }
        }
        _ => {}
    }
    let down = current + Position::DOWN;
    match down.access(&pipe_map) {
        Some(VERTICAL) | Some(SOUTH_WEST) | Some(SOUTH_EAST) => {
            if !visited.contains(&down) {
                *down.access_mut(&mut steps_map).unwrap() = current.access(&steps_map).unwrap() + 1;
                visited.insert(down);
                stack.push(down)
            }
        }
        _ => {}
    }
    while let Some(current) = stack.pop() {
        let current_pipe = current.access(&pipe_map).unwrap();
        let directions = match current_pipe {
            HORIZONTAL => [Position::LEFT, Position::RIGHT],
            VERTICAL => [Position::UP, Position::DOWN],
            NORTH_EAST => [Position::UP, Position::RIGHT],
            NORTH_WEST => [Position::UP, Position::LEFT],
            SOUTH_EAST => [Position::DOWN, Position::RIGHT],
            SOUTH_WEST => [Position::DOWN, Position::LEFT],
            _ => { continue; }
        };
        let mut contains_all = true;
        for direction in directions {
            let new_position = current + direction;
            if !visited.contains(&new_position) {
                *new_position.access_mut(&mut steps_map).unwrap() = current.access(&steps_map).unwrap() + 1;
                visited.insert(new_position);
                stack.push(new_position);
                contains_all = false;
            }
        }
        if contains_all {
            println!("ok {}", current.access(&steps_map).unwrap());
        }
    }
    let silver = steps_map.iter().map(|x| x.iter().max().unwrap()).max().unwrap();
    for row in &steps_map {
        for steps in row {
            eprint!("{: >2} ", steps);
        }
        eprintln!();
    }
    println!("silver: {}", silver / 2 +1);
}
