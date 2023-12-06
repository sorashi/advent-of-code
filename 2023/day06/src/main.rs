use std::io::stdin;

fn hold_times(d: u64, t: u64) -> (f64, f64) {
    let discrim: f64 = (t as f64 * t as f64 - 4f64 * d as f64).into();
    let h1: f64 = (t as f64 - discrim.sqrt()) / 2.0;
    let h2: f64 = (t as f64 + discrim.sqrt()) / 2.0;
    (h1, h2)
}

fn get_ways_count(h1: f64, mut h2: f64, t: u64) -> u64 {
    let h1 = (h1 as u64) + 1;
    if h2 == h2.floor() {
        h2 -= 1.0;
    }
    (h2.floor() as u64).min(t) - h1.max(0) + 1
}

fn main() {
    let mut times = String::new();
    let mut distances = String::new();
    stdin().read_line(&mut times).unwrap();
    stdin().read_line(&mut distances).unwrap();
    let time_gold: u64 = times[5..].replace(" ", "").trim().parse().unwrap();
    let distance_gold: u64 = distances[9..].replace(" ", "").trim().parse().unwrap();
    let times: Vec<_> = times[5..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let distances: Vec<_> = distances[9..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let mut silver = 0;
    for i in 0..times.len() {
        let t = times[i];
        let d = distances[i];
        let (h1, h2) = hold_times(d, t);
        let ways = get_ways_count(h1, h2, t);
        if silver == 0 {
            silver = ways
        } else {
            silver *= ways
        };
    }
    let (h1, h2) = hold_times(distance_gold, time_gold);
    let gold = get_ways_count(h1, h2, time_gold);
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
