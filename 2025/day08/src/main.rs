use std::{collections::HashMap, io::stdin};

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

struct UnionFind {
    colors: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            colors: (0..n).collect(),
        }
    }
    fn union(&mut self, i: usize, j: usize) {
        let color = self.colors[i];
        let other_color = self.colors[j];
        for c in self.colors.iter_mut() {
            if *c == other_color {
                *c = color;
            }
        }
    }
    fn find(&self, i: usize, j: usize) -> bool {
        self.colors[i] == self.colors[j]
    }
    fn silver(&self) -> u64 {
        let mut hm = HashMap::<usize, usize>::new();
        for color in &self.colors {
            let cnt = hm.entry(*color).or_default();
            *cnt += 1;
        }
        let mut vals: Vec<_> = hm.into_values().collect();
        vals.sort_unstable_by(|a, b| b.cmp(a));
        vals.into_iter().take(3).map(|v| v as u64).product()
    }
}

fn solve(boxes: &[Jbox], silver_take: usize) -> (u64, u64) {
    let mut uf = UnionFind::new(boxes.len());
    let edges = get_edges(boxes);
    for (i, j) in &edges[0..silver_take] {
        if !uf.find(*i, *j) {
            uf.union(*i, *j);
        }
    }
    let silver = uf.silver();
    let mut last_union = (0, 0);
    for (i, j) in &edges[silver_take..] {
        if !uf.find(*i, *j) {
            uf.union(*i, *j);
            last_union = (*i, *j);
        }
    }
    let gold = boxes[last_union.0].x * boxes[last_union.1].x;
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

fn get_edges(boxes: &[Jbox]) -> Vec<(usize, usize)> {
    let mut edges = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            edges.push((i, j));
        }
    }
    edges.sort_unstable_by(|a, b| {
        let dist1 = boxes[a.0].dist(&boxes[a.1]);
        let dist2 = boxes[b.0].dist(&boxes[b.1]);
        dist1.partial_cmp(&dist2).unwrap()
    });
    edges
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
    fn test_edges() {
        let boxes = get_example_boxes();
        let edges = get_edges(&boxes);

        let pair = edges[0];
        let box1 = &boxes[pair.0];
        let box2 = &boxes[pair.1];
        let box1_expected = Jbox::new(162, 817, 812);
        let box2_expected = Jbox::new(425, 690, 689);
        assert!(box1 == &box1_expected || box1 == &box2_expected);
        assert!(box2 == &box1_expected || box2 == &box2_expected);

        let pair = edges[1];
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
