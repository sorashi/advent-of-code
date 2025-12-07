use std::io::stdin;

fn main() {
    let mut silver = 0;
    let mut state = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        if state.is_empty() {
            state.resize(line.len(), 0u64);
        }
        for (i, c) in line.as_bytes().iter().enumerate() {
            match *c {
                b'S' => {
                    state[i] = 1;
                }
                b'^' => {
                    if state[i] > 0 {
                        // to be more general, we should save the changes to distribute at the end
                        // of this level, but this solution works
                        silver += 1;
                        let timelines = state[i];
                        state[i] = 0;
                        state[i - 1] += timelines;
                        state[i + 1] += timelines;
                    }
                }
                _ => continue,
            }
        }
    }
    println!("silver: {}", silver);
    println!("gold: {}", state.iter().sum::<u64>());
}
