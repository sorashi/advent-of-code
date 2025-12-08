use std::{collections::HashSet, fmt::Display, io::stdin};

use itertools::Itertools;

#[derive(Debug)]
struct Jbox {
    x: u64,
    y: u64,
    z: u64,
}
impl Jbox {
    fn dist(&self, other: &Jbox) -> f32 {
        ((self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)) as f32)
            .sqrt()
    }
}
impl Display for Jbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn main() {
    let mut boxes = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let coors = line
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        boxes.push(Jbox {
            x: coors[0],
            y: coors[1],
            z: coors[2],
        });
    }
    let mut colors = (0..boxes.len()).collect::<Vec<usize>>();
    let mut pair_finder = PairFinder::new(&boxes);
    for (i, j) in (&mut pair_finder).take(1000) {
        println!(
            "{i} {j} | {} {} {}",
            boxes[i],
            boxes[j],
            boxes[i].dist(&boxes[j]),
        );
        let color = colors[i];
        let other_color = colors[j];
        for k in colors.iter_mut() {
            if *k == other_color {
                *k = color;
            }
        }
    }
    let groups = colors
        .iter()
        .into_grouping_map_by(|c| *c)
        .fold(0u64, |acc, _key, _val| acc + 1);
    for (k, v) in groups.iter().sorted_unstable_by_key(|(_, v)| **v).rev() {
        println!("group {k}: {v}");
    }
    let silver = groups
        .values()
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<u64>();
    loop {
        let (i, j) = pair_finder.next().unwrap();
        let color = colors[i];
        let other_color = colors[j];
        for k in colors.iter_mut() {
            if *k == other_color {
                *k = color;
            }
        }
        if colors.iter().all(|c| *c == color) {
            println!("{} {}", boxes[i].x, boxes[j].x);
            break;
        }
    }
    println!("silver: {}", silver);
}

struct PairFinder<'a> {
    returned: HashSet<(usize, usize)>,
    boxes: &'a [Jbox],
}
impl<'a> PairFinder<'a> {
    fn new(boxes: &'a [Jbox]) -> Self {
        PairFinder {
            returned: HashSet::new(),
            boxes,
        }
    }
}
impl<'a> Iterator for PairFinder<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let mut found = false;
        let mut min_dist = f32::INFINITY;
        let mut pair = (0, 0);
        for i in 0..self.boxes.len() {
            for j in i + 1..self.boxes.len() {
                if self.returned.contains(&(i, j)) {
                    continue;
                }
                let cur_dist = self.boxes[i].dist(&self.boxes[j]);
                if cur_dist < min_dist {
                    min_dist = cur_dist;
                    pair = (i, j);
                    found = true;
                }
            }
        }
        if found {
            self.returned.insert(pair);
            Some(pair)
        } else {
            None
        }
    }
}
