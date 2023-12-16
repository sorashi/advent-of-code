use std::arch::x86_64::_mm_minpos_epu16;
use std::collections::HashSet;
use std::io::{stdin, Read};
use std::ops::{Add, AddAssign};

type N = isize;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Vector {
    x: N,
    y: N,
}

impl Vector {
    const UP: Vector = Vector { x: 0, y: -1 };
    const DOWN: Vector = Vector { x: 0, y: 1 };
    const LEFT: Vector = Vector { x: -1, y: 0 };
    const RIGHT: Vector = Vector { x: 1, y: 0 };
    const ZERO: Vector = Vector { x: 0, y: 0 };

    fn get<'a, T, A: AsRef<[T]>>(&self, array: &'a [A]) -> &'a T {
        &array[self.y as usize].as_ref()[self.x as usize]
    }
    fn set<'a, T, A: AsMut<[T]>>(&self, array: &mut [A], value: T) {
        array[self.y as usize].as_mut()[self.x as usize] = value;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct DirPos {
    direction: Vector,
    position: Vector,
}

struct Beam {
    history: Vec<DirPos>,
    dir_pos: DirPos,
}

impl Beam {
    fn new(position: Vector, direction: Vector) -> Self {
        Self {
            history: vec![],
            dir_pos: DirPos {
                position,
                direction,
            },
        }
    }
    fn step<S: AsRef<[u8]>>(&mut self, map: &[S]) -> (Vector, Option<Beam>) {
        let mut new_beam = None;
        match *self.dir_pos.position.get(map) {
            b'/' => {
                self.dir_pos.direction = Vector {
                    x: -self.dir_pos.direction.y,
                    y: -self.dir_pos.direction.x,
                }
            }
            b'\\' => {
                self.dir_pos.direction = Vector {
                    x: self.dir_pos.direction.y,
                    y: self.dir_pos.direction.x,
                };
            }
            b'|' => {
                if self.dir_pos.direction.x != 0 {
                    self.dir_pos.direction = Vector::UP;
                    new_beam = Some(Beam::new(self.dir_pos.position, Vector::DOWN));
                }
            }
            b'-' => {
                if self.dir_pos.direction.y != 0 {
                    self.dir_pos.direction = Vector::LEFT;
                    new_beam = Some(Beam::new(self.dir_pos.position, Vector::RIGHT));
                }
            }
            b'.' => {}
            _ => panic!(),
        }
        self.dir_pos.position += self.dir_pos.direction;
        (self.dir_pos.position, new_beam)
    }
}

fn print_energized(energized: &Vec<Vec<bool>>) {
    for row in energized {
        for &e in row {
            eprint!("{}", if e { '#' } else { '.' });
        }
        eprintln!();
    }
}

fn get_energized_tile_count(first_beam: Beam, map: &Vec<&[u8]>) -> usize {
    let mut beams = vec![first_beam];
    let width = map[0].len() as isize;
    let height = map.len() as isize;
    let mut energized = vec![vec![false; width as usize]; height as usize];
    energized[0][0] = true;
    let mut visited: HashSet<DirPos> = HashSet::new();
    visited.insert(beams[0].dir_pos);
    while !beams.is_empty() {
        let mut new_beams = vec![];
        let mut beams_for_removal = vec![];
        for (i, beam) in beams.iter_mut().enumerate() {
            let (new_pos, new_beam) = beam.step(&map);
            if let Some(new_beam) = new_beam {
                if !visited.contains(&new_beam.dir_pos) {
                    visited.insert(new_beam.dir_pos);
                    new_beams.push(new_beam);
                }
            }
            if new_pos.x < 0
                || new_pos.y < 0
                || new_pos.x >= width
                || new_pos.y >= height
                || visited.contains(&beam.dir_pos)
            {
                beams_for_removal.push(i);
                continue;
            }
            visited.insert(beam.dir_pos);
            new_pos.set(&mut energized, true);
        }
        beams_for_removal.sort();
        for i in beams_for_removal.iter().rev() {
            beams.remove(*i);
        }
        beams.extend(new_beams);
    }
    let mut answer = 0;
    for row in energized {
        answer += row.iter().filter(|x| **x).count();
    }
    answer
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input
        .split_terminator('\n')
        .map(|x| x.trim().as_bytes())
        .collect();
    println!("silver: {}", get_energized_tile_count(Beam::new(Vector::ZERO, Vector::RIGHT), &lines));

    let width = lines[0].len() as isize;
    let height = lines.len() as isize;
    let mut gold = 0;
    for y in 0..height {
        gold = gold.max(get_energized_tile_count(Beam::new(Vector { x: 0, y }, Vector::RIGHT), &lines));
        gold = gold.max(get_energized_tile_count(Beam::new(Vector { x: width - 1, y }, Vector::LEFT), &lines));
    }
    for x in 0..width {
        gold = gold.max(get_energized_tile_count(Beam::new(Vector { x, y: 0 }, Vector::DOWN), &lines));
        gold = gold.max(get_energized_tile_count(Beam::new(Vector { x, y: height - 1 }, Vector::UP), &lines));
    }
    println!("gold: {}", gold);
}
