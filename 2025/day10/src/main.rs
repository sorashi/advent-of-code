use std::{collections::VecDeque, io::stdin};

use z3::{Optimize, ast::Int};

fn main() {
    let mut silver = 0;
    let mut gold = 0;
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
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut buttons = vec![];
        for but in revspl {
            let button = but[1..but.len() - 1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            buttons.push(button);
        }
        buttons.sort_unstable();
        silver += solve_silver(&target, &buttons);
        gold += solve_gold(&joltages, &buttons);
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}

fn transfer_silver(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut new_state = state.to_vec();
    for i in button {
        new_state[*i] = !new_state[*i];
    }
    new_state
}

fn solve_silver(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let n = target.len();
    let mut queue = VecDeque::new();
    queue.push_back((vec![false; n], 0));
    while let Some((state, depth)) = queue.pop_front() {
        for button in buttons {
            let new_state = transfer_silver(&state, button);
            if new_state == target {
                return depth + 1;
            }
            queue.push_back((new_state, depth + 1));
        }
    }
    panic!("No solution found");
}

fn solve_gold(target: &[usize], buttons: &[Vec<usize>]) -> usize {
    let vars = (0..buttons.len())
        .map(|i| Int::new_const(format!("x_{}", i)))
        .collect::<Vec<_>>();
    let solver = Optimize::new();
    solver.minimize(&Int::add(&vars));
    for var in &vars {
        solver.assert(&var.ge(Int::from_u64(0)));
    }
    for i in 0..target.len() {
        let mut included_vars = vec![];
        for j in 0..buttons.len() {
            if buttons[j].contains(&i) {
                included_vars.push(&vars[j]);
            }
        }
        let sum = Int::add(&included_vars).eq(Int::from_u64(target[i] as u64));
        solver.assert(&sum);
    }

    match solver.check(&[]) {
        z3::SatResult::Sat => (),
        _ => panic!("No solution found"),
    };
    let m = solver.get_model().unwrap();
    vars.iter()
        .map(|v| m.get_const_interp(v).unwrap().as_u64().unwrap() as usize)
        .sum()
}
