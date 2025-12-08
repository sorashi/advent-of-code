use std::{collections::HashSet, fmt::Display, io::stdin};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Jbox {
    x: u64,
    y: u64,
    z: u64,
}
impl Jbox {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Jbox { x, y, z }
    }
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

fn solve(boxes: &[Jbox], silver_take: usize) -> (u64, u64) {
    let mut colors = (0..boxes.len()).collect::<Vec<usize>>();
    let mut pair_finder = PairFinder::new(&boxes);
    for (i, j) in (&mut pair_finder).take(silver_take) {
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
    let silver = groups
        .values()
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<u64>();
    let gold;
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
            gold = boxes[i].x * boxes[j].x;
            break;
        }
    }
    (silver, gold)
}

fn main() {
    let mut boxes = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let coors = line
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        boxes.push(Jbox::new(coors[0], coors[1], coors[2]));
    }
    let (silver, gold) = solve(&boxes, 1000);
    println!("silver: {silver}");
    println!("gold: {gold}");
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

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_boxes() -> Vec<Jbox> {
        vec![
            Jbox::new(162, 817, 812),
            Jbox::new(57, 618, 57),
            Jbox::new(906, 360, 560),
            Jbox::new(592, 479, 940),
            Jbox::new(352, 342, 300),
            Jbox::new(466, 668, 158),
            Jbox::new(542, 29, 236),
            Jbox::new(431, 825, 988),
            Jbox::new(739, 650, 466),
            Jbox::new(52, 470, 668),
            Jbox::new(216, 146, 977),
            Jbox::new(819, 987, 18),
            Jbox::new(117, 168, 530),
            Jbox::new(805, 96, 715),
            Jbox::new(346, 949, 466),
            Jbox::new(970, 615, 88),
            Jbox::new(941, 993, 340),
            Jbox::new(862, 61, 35),
            Jbox::new(984, 92, 344),
            Jbox::new(425, 690, 689),
        ]
    }
    #[test]
    fn test_pairfinder() {
        let boxes = get_example_boxes();
        let mut pair_finder = PairFinder::new(&boxes);
        let iter = &mut pair_finder;

        let pair = iter.next().unwrap();
        let box1 = &boxes[pair.0];
        let box2 = &boxes[pair.1];
        let box1_expected = Jbox::new(162, 817, 812);
        let box2_expected = Jbox::new(425, 690, 689);
        assert!(box1 == &box1_expected || box1 == &box2_expected);
        assert!(box2 == &box1_expected || box2 == &box2_expected);

        let pair = iter.next().unwrap();
        let box1 = &boxes[pair.0];
        let box2 = &boxes[pair.1];
        let box1_expected = Jbox::new(162, 817, 812);
        let box2_expected = Jbox::new(431, 825, 988);
        assert!(box1 == &box1_expected || box1 == &box2_expected);
        assert!(box2 == &box1_expected || box2 == &box2_expected);
    }
    #[test]
    fn test_silver() {
        let boxes = get_example_boxes();
        let (silver, _) = solve(&boxes, 10);
        assert_eq!(silver, 40);
    }
    #[test]
    fn test_gold() {
        let boxes = get_example_boxes();
        let (_, gold) = solve(&boxes, 10);
        assert_eq!(gold, 25272);
    }
}
