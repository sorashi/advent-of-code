use std::{io::stdin, ops::Range};

struct Map {
    mapped_ranges: Vec<MappedRange>,
}

impl Map {
    fn new() -> Self {
        Map {
            mapped_ranges: vec![],
        }
    }

    fn transform(&self, num: usize) -> usize {
        for mapped_range in &self.mapped_ranges {
            if mapped_range.source.contains(&num) {
                return mapped_range.destination.start + (num - mapped_range.source.start);
            }
        }
        return num;
    }
}

struct MappedRange {
    source: Range<usize>,
    destination: Range<usize>,
}

fn parse_map() -> Option<Map> {
    let mut map_name = String::new();
    stdin().read_line(&mut map_name).unwrap();
    let mut map = Map::new();

    for line in stdin()
        .lines()
        .take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
    {
        let line = line.unwrap();
        let [destination_start, source_start, length] = line
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()[..]
        else {
            unreachable!()
        };
        map.mapped_ranges.push(MappedRange {
            source: (source_start..source_start + length),
            destination: (destination_start..destination_start + length),
        })
    }
    if map.mapped_ranges.is_empty() {
        return None;
    }
    return Some(map);
}

fn transform_seed(seed: usize, maps: &Vec<Map>) -> usize {
    let mut current = seed;
    for map in maps {
        current = map.transform(current);
    }
    current
}

fn main() {
    let mut seeds = String::new();
    stdin().read_line(&mut seeds).unwrap();
    let colon = seeds.find(':').unwrap();
    let seeds: Vec<usize> = seeds[(colon + 1)..]
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect();

    stdin().read_line(&mut String::new()).unwrap();
    let mut maps: Vec<Map> = vec![];
    while let Some(map) = parse_map() {
        maps.push(map);
    }

    let mut silver = usize::MAX;
    let mut gold = usize::MAX;
    for slice in seeds.chunks(2) {
        let [seed_start, length] = *slice else {
            unreachable!()
        };
        silver = std::cmp::min(silver, transform_seed(seed_start, &maps));
        silver = std::cmp::min(silver, transform_seed(length, &maps));
        for seed in seed_start..(seed_start + length) {
            gold = std::cmp::min(gold, transform_seed(seed, &maps));
        }
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
