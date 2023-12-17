use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::{stdin, Read};
use aoc_utils::{TwoDimArray, Vector};

#[derive(Hash, Eq, PartialEq)]
struct CrucibleStateWithoutHeatLoss {
    position: Vector,
    direction: Vector,
    consecutive: usize,
}

impl CrucibleStateWithoutHeatLoss {
    fn clone_from_crucible_state(crucible_state: &CrucibleState) -> Self {
        Self {
            position: crucible_state.position,
            direction: crucible_state.direction,
            consecutive: crucible_state.consecutive,
        }
    }
}

struct CrucibleState {
    position: Vector,
    direction: Vector,
    consecutive: usize,
    heat_loss: usize,
}

impl Eq for CrucibleState {}

impl PartialEq<Self> for CrucibleState {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss.eq(&other.heat_loss)
    }
}

impl PartialOrd<Self> for CrucibleState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CrucibleState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let loss_map = input
        .split_terminator('\n')
        .map(|row| row.as_bytes().iter().map(|c| c - 0x30).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let loss_map = TwoDimArray::from(loss_map);
    let mut seen: HashSet<CrucibleStateWithoutHeatLoss> = HashSet::new();
    let mut pq: BinaryHeap<CrucibleState> = BinaryHeap::new();
    let destination = Vector { x: loss_map.width() as isize - 1, y: loss_map.height() as isize - 1 };
    let mut silver = 0;
    let first_state = CrucibleState {
        position: Vector::ZERO,
        direction: Vector::ZERO,
        heat_loss: 0,
        consecutive: 0
    };
    pq.push(first_state);
    while let Some(current) = pq.pop() {
        let state_without_hl = CrucibleStateWithoutHeatLoss::clone_from_crucible_state(&current);
        if current.position == destination {
            silver = current.heat_loss;
            break;
        }
        if seen.contains(&state_without_hl) {
            continue;
        }
        seen.insert(state_without_hl);
        if current.consecutive < 3 && current.direction != Vector::ZERO {
            let new_pos = current.position + current.direction;
            if loss_map.is_vector_in_bounds(&new_pos) {
                let new_state = CrucibleState {
                    position: new_pos,
                    direction: current.direction,
                    heat_loss: current.heat_loss + *loss_map.get_by_vector(&new_pos).unwrap() as usize,
                    consecutive: current.consecutive + 1,
                };
                pq.push(new_state);
            }
        }
        for dir in [Vector::LEFT, Vector::RIGHT, Vector::UP, Vector::DOWN] {
            if -dir == current.direction || dir == current.direction {
                continue;
            }
            let new_pos = current.position + dir;
            if !loss_map.is_vector_in_bounds(&new_pos) {
                continue;
            }
            let new_state = CrucibleState {
                position: new_pos,
                direction: dir,
                heat_loss: current.heat_loss + *loss_map.get_by_vector(&new_pos).unwrap() as usize,
                consecutive: 1,
            };
            pq.push(new_state);
        }
    }
    println!("silver: {}", silver);
}
