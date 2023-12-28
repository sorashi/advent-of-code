use std::io::{stdin, Read};
use Rps::*;
use RpsResult::*;

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum RpsResult {
    Win,
    Loss,
    Draw,
}

fn get_loss(them: Rps) -> Rps {
    match them {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}
fn get_win(them: Rps) -> Rps {
    match them {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}
fn get_draw(them: Rps) -> Rps {
    them
}

fn get_score(us: Rps, them: Rps) -> usize {
    let result = match (us, them) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Loss,
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Loss,
        (Scissors, Rock) => Loss,
        (Scissors, Paper) => Win,
        (Scissors, Scissors) => Draw,
    };
    (match us {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    } + match result {
        Loss => 0,
        Draw => 3,
        Win => 6,
    })
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut silver = 0;
    let mut gold = 0;
    for round in input.split_terminator('\n') {
        let (them, us) = round.split_once(' ').unwrap();
        let us = match us {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!(),
        };
        let them = match them {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        };
        silver += get_score(us, them);
        let us = match us {
            Rock => get_loss(them),
            Paper => get_draw(them),
            Scissors => get_win(them),
        };
        gold += get_score(us, them);
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
