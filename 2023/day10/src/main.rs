use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::Into;
use std::io::{stdin, Read};
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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: N,
    y: N,
}

impl Position {
    const LEFT: Position = Position { x: -1, y: 0 };
    const RIGHT: Position = Position { x: 1, y: 0 };
    const UP: Position = Position { x: 0, y: -1 };
    const DOWN: Position = Position { x: 0, y: 1 };
    fn access<T, U>(&self, map: &[U]) -> Option<T>
    where
        T: Copy,
        U: AsRef<[T]>,
    {
        map.get(self.y as usize)
            .and_then(|row| row.as_ref().get(self.x as usize).copied())
    }
    fn access_mut<'a, T>(&self, map: &'a mut Vec<Vec<T>>) -> Option<&'a mut T>
    where
        T: Copy,
    {
        map.get_mut(self.y as usize)
            .and_then(|row| row.get_mut(self.x as usize))
    }
}

impl From<(N, N)> for Position {
    fn from((x, y): (N, N)) -> Self {
        Position { x, y }
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
    let mut steps_map: Vec<Vec<isize>> = vec![vec![-1; pipe_map[0].len()]; pipe_map.len()];
    let mut path = vec![];
    *start.access_mut(&mut steps_map).unwrap() = 0;

    let left = start + Position::LEFT;
    let right = start + Position::LEFT;
    let up = start + Position::UP;
    let down = start + Position::DOWN;
    if let Some(pos) = left.access(&pipe_map) {
        if [SOUTH_EAST, NORTH_EAST, HORIZONTAL].contains(&pos) && stack.is_empty() {
            stack.push(left);
        }
    }
    if let Some(pos) = right.access(&pipe_map) {
        if [SOUTH_WEST, NORTH_WEST, HORIZONTAL].contains(&pos) && stack.is_empty() {
            stack.push(right);
        }
    }
    if let Some(pos) = up.access(&pipe_map) {
        if [NORTH_EAST, NORTH_WEST, VERTICAL].contains(&pos) && stack.is_empty() {
            stack.push(up);
        }
    }
    if let Some(pos) = down.access(&pipe_map) {
        if [SOUTH_EAST, SOUTH_WEST, VERTICAL].contains(&pos) && stack.is_empty() {
            stack.push(down);
        }
    }
    let first = stack[0];
    visited.insert(first);
    *first.access_mut(&mut steps_map).unwrap() = 1;
    path.push(start);
    path.push(first);

    while let Some(current) = stack.pop() {
        let current_pipe = current.access(&pipe_map).unwrap();
        let directions = match current_pipe {
            HORIZONTAL => vec![Position::LEFT, Position::RIGHT],
            VERTICAL => vec![Position::UP, Position::DOWN],
            NORTH_EAST => vec![Position::UP, Position::RIGHT],
            NORTH_WEST => vec![Position::UP, Position::LEFT],
            SOUTH_EAST => vec![Position::DOWN, Position::RIGHT],
            SOUTH_WEST => vec![Position::DOWN, Position::LEFT],
            _ => {
                continue;
            }
        };
        for direction in directions {
            let new_position = current + direction;
            if !visited.contains(&new_position) {
                *new_position.access_mut(&mut steps_map).unwrap() =
                    current.access(&steps_map).unwrap() + 1;
                visited.insert(new_position);
                stack.push(new_position);
                path.push(new_position);
            }
        }
    }
    let silver = steps_map
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap();

    let mut gold = 0;
    let height = pipe_map.len();
    let width = pipe_map[0].len();
    let top_left = *path
        .iter()
        .min_by(|a, b| match a.x.cmp(&b.x) {
            Ordering::Equal => a.y.cmp(&b.y),
            ord => ord,
        })
        .unwrap();
    let bottom_right = *path
        .iter()
        .max_by(|a, b| match a.x.cmp(&b.x) {
            Ordering::Equal => a.y.cmp(&b.y),
            ord => ord,
        })
        .unwrap();

    for j in 0..height {
        let mut outside = true;
        let mut last_changer: Option<u8> = None;
        for i in 0..width {
            let current = Position {
                x: i as N,
                y: j as N,
            };
            let current_pipe = current.access(&pipe_map).unwrap();
            if path.contains(&current) {
                match current_pipe {
                    VERTICAL => {
                        outside = !outside;
                        last_changer = None;
                    }
                    NORTH_EAST | NORTH_WEST | SOUTH_EAST | SOUTH_WEST => {
                        if let Some(last) = last_changer {
                            match (last, current_pipe) {
                                (NORTH_WEST, SOUTH_EAST) => outside = !outside,
                                (SOUTH_EAST, NORTH_WEST) => outside = !outside,
                                (NORTH_EAST, SOUTH_WEST) => outside = !outside,
                                (SOUTH_WEST, NORTH_EAST) => outside = !outside,
                                _ => {}
                            }
                            last_changer = None
                        } else {
                            last_changer = Some(current_pipe);
                        }
                    }
                    _ => {}
                }
            } else if !outside {
                gold += 1;
            }
        }
    }
    println!("silver: {}", silver / 2 + 1);
    println!("gold: {}", gold);
}
