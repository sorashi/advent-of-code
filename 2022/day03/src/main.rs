use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn get_priority(item: u8) -> u8 {
    if item >= b'a' {
        item - b'a' + 1
    } else {
        item - b'A' + 27
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut silver = 0usize;
    let mut gold = 0;

    'rucksack: for rucksack in input.split_terminator('\n') {
        let compartments = rucksack.as_bytes().split_at(rucksack.len() / 2);
        let compartment1 = compartments.0.iter().copied().collect::<HashSet<u8>>();
        for item in compartments.1 {
            if compartment1.contains(item) {
                silver += get_priority(*item) as usize;
                continue 'rucksack;
            }
        }
    }

    let lines: Vec<_> = input.split_terminator('\n').collect();
    for rucksacks in lines.chunks_exact(3) {
        let mut rucksacks = rucksacks
            .into_iter()
            .map(|r| r.as_bytes().into_iter().copied().collect::<HashSet<u8>>())
            .collect::<Vec<_>>();
        let (first, others) = rucksacks.split_at_mut(1);
        let first = &mut first[0];
        for other in others {
            first.retain(|x| other.contains(x));
        }
        assert_eq!(first.len(), 1);
        gold += get_priority(*first.iter().next().unwrap()) as usize;
    }

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
