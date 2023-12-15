use std::io::{stdin, Read};

fn hash(input: &[u8]) -> u8 {
    let mut current: u8 = 0;
    for &c in input {
        current = current.wrapping_add(c);
        current = current.wrapping_mul(17);
    }
    current
}

#[derive(Clone)]
struct Lens<'k> {
    label: &'k str,
    value: u64,
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut silver: u64 = 0;
    for step in input.split(',').map(|x| x.as_bytes()) {
        silver += hash(step) as u64;
    }
    println!("silver: {}", silver);

    let mut boxes = vec![vec![]; 256];
    for lens in input.split(',') {
        let removal = lens.rfind('-');
        if let Some(removal) = removal {
            let lens = &lens[..removal];
            let lens_bytes = lens.as_bytes();
            let hash = hash(lens_bytes);
            let index = boxes[hash as usize]
                .iter()
                .position(|x: &Lens| x.label == lens);
            if let Some(index) = index {
                boxes[hash as usize].remove(index);
            }
        } else {
            let (lens, value) = lens.split_once('=').unwrap();
            let value = value.parse().unwrap();
            let lens_bytes = lens.as_bytes();
            let hash = hash(lens_bytes);
            let index = boxes[hash as usize].iter().position(|x| x.label == lens);
            if let Some(index) = index {
                boxes[hash as usize][index].value = value;
            } else {
                boxes[hash as usize].push(Lens { label: lens, value });
            }
        }
    }

    let mut gold = 0u64;
    for (i, the_box) in boxes.iter().enumerate() {
        for (j, lens) in the_box.iter().enumerate() {
            let power: u64 = (i + 1) as u64 * (j + 1) as u64 * lens.value;
            gold += power;
        }
    }
    println!("gold: {}", gold);
}
