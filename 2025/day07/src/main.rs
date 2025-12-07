use std::{io::stdin, iter};

fn main() {
    let mut silver = 0;
    let mut hs = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        if hs.len() == 0 {
            hs.reserve(line.len());
            hs.extend(iter::repeat_n(0u64, line.len()));
        }
        for (i, c) in line.as_bytes().iter().enumerate() {
            match *c {
                b'S' => {
                    hs[i] = 1;
                }
                b'^' => {
                    if hs[i] > 0 {
                        silver += 1;
                        let timelines = hs[i];
                        hs[i] = 0;
                        hs[i - 1] += timelines;
                        hs[i + 1] += timelines;
                    }
                }
                _ => continue,
            }
        }
    }
    println!("silver: {}", silver);
    println!("gold: {}", hs.iter().sum::<u64>());
}
