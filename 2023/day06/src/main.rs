use std::io::stdin;

fn hold_times(d: i32, t: i32) -> (f64, f64) {
    let discrim: f64 = (t * t - 4 * d).into();
    let h1: f64 = (t as f64 - discrim.sqrt()) / 2.0;
    let h2: f64 = (t as f64 + discrim.sqrt()) / 2.0;
    (h1, h2)
}

fn get_ways_count(h1: f64, mut h2: f64, t: i32) -> i32 {
    let h1 = (h1 as i32) + 1;
    if h2 == h2.floor() {
        h2 -= 1.0;
    }
    (h2.floor() as i32).min(t) - h1.max(0) + 1
}

fn main() {
    let mut times = String::new();
    let mut distances = String::new();
    stdin().read_line(&mut times).unwrap();
    stdin().read_line(&mut distances).unwrap();
    let times: Vec<_> = times[5..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let distances: Vec<_> = distances[9..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    assert_eq!(times.len(), distances.len());

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
    println!("silver: {}", silver);
}
