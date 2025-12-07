use std::{collections::HashSet, io::stdin};

fn main() {
    let mut silver = 0;
    let mut hs = HashSet::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        for (i, c) in line.as_bytes().iter().enumerate() {
            match *c {
                b'S' => {
                    hs.insert(i);
                }
                b'^' => {
                    if hs.contains(&i) {
                        hs.remove(&i);
                        silver += 1;
                        hs.insert(i - 1);
                        hs.insert(i + 1);
                    }
                }
                _ => continue,
            }
        }
    }
    println!("silver: {}", silver);
}
