use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (input, _) = input.split_once('\n').unwrap();

    let mut silver: u64 = 0;
    for step in input.split(',').map(|x|x.as_bytes()) {
        let mut current: u8 = 0;
        for &c in step {
            current = current.wrapping_add(c);
            current = current.wrapping_mul(17);
        }
        silver += current as u64;
    }
    println!("silver: {}", silver);
}
