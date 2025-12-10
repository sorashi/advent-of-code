use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut silver = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let mut spl = line.split(' ');
        let target = spl.next().unwrap();
        let target = target[1..target.len() - 1]
            .as_bytes()
            .iter()
            .map(|b| *b == b'#')
            .collect::<Vec<bool>>();
        let mut revspl = spl.rev();
        let joltages = revspl.next().unwrap();
        let joltages = joltages[1..joltages.len() - 1]
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut buttons = vec![];
        for but in revspl {
            let button = but[1..but.len() - 1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            buttons.push(button);
        }
        silver += solve(&target, &buttons);
    }
    println!("silver: {}", silver);
}

fn transfer(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut new_state = state.to_vec();
    for i in button {
        new_state[*i] = !new_state[*i];
    }
    new_state
}

fn solve(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let n = target.len();
    let mut queue = VecDeque::new();
    queue.push_back((vec![false; n], 0));
    while let Some((state, depth)) = queue.pop_front() {
        for button in buttons {
            let new_state = transfer(&state, button);
            if new_state == target {
                return depth + 1;
            }
            queue.push_back((new_state, depth + 1));
        }
    }
    panic!("No solution found");
}
