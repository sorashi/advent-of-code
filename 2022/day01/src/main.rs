use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut elf_cals: Vec<_> = input
        .split_terminator("\n\n")
        .map(|elf| {
            elf.split_terminator('\n')
                .map(|cal| cal.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();
    let silver = *elf_cals.iter().max().unwrap();

    elf_cals.sort_unstable();
    elf_cals.reverse();
    let gold: usize = elf_cals.iter().take(3).sum();

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
