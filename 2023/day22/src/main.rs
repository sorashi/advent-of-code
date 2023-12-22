use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

#[derive(Copy, Clone, Debug)]
struct Vector3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Eq for Vector3 {}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z
    }
}

impl PartialOrd for Vector3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.z.partial_cmp(&other.z)
    }
}

impl Ord for Vector3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Box {
    from: Vector3,
    to: Vector3,
}

impl Box {
    fn overlaps(&self, other: &Box) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() >= other.bottom()
            && self.bottom() <= other.top()
    }
    fn left(&self) -> usize {
        self.from.x.min(self.to.x)
    }
    fn right(&self) -> usize {
        self.from.x.max(self.to.x)
    }
    fn top(&self) -> usize {
        self.from.y.max(self.to.y)
    }
    fn bottom(&self) -> usize {
        self.from.y.min(self.to.y)
    }
    fn roof(&self) -> usize {
        self.from.z.max(self.to.z)
    }
    fn floor(&self) -> usize {
        self.from.z.min(self.to.z)
    }
    fn roof_mut(&mut self) -> &mut usize {
        if self.from.z <= self.to.z {
            return &mut self.to.z;
        } else {
            return &mut self.from.z;
        }
    }
    fn floor_mut(&mut self) -> &mut usize {
        if self.from.z <= self.to.z {
            return &mut self.from.z;
        } else {
            return &mut self.to.z;
        }
    }
}

fn parse_vector(s: &str) -> Vector3 {
    let coors: Vec<_> = s.split(',').map(|x| x.parse().unwrap()).collect();
    Vector3 {
        x: coors[0],
        y: coors[1],
        z: coors[2],
    }
}

fn main() {
    let mut bricks: Vec<Box> = vec![];
    for line in stdin().lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once('~').unwrap();
        let from = parse_vector(from);
        let to = parse_vector(to);
        bricks.push(Box { from, to });
    }

    bricks.sort_by_key(|x| x.from.min(x.to));

    for i in 0..bricks.len() {
        let mut current_max_z = 1;
        let brick = &bricks[i];
        for j in 0..i {
            let check = &bricks[j];
            if check.overlaps(brick) {
                current_max_z = current_max_z.max(check.roof() + 1)
            }
        }
        let brick = &mut bricks[i];
        let (floor_mut, roof_mut) = if brick.to.z <= brick.from.z {
            (&mut brick.to, &mut brick.from)
        } else {
            (&mut brick.from, &mut brick.to)
        };
        roof_mut.z += current_max_z;
        roof_mut.z -= floor_mut.z;
        floor_mut.z = current_max_z;
    }

    bricks.sort_by_key(|x| x.from.min(x.to));

    for brick in &bricks {
        eprintln!(
            "{} {} {} ~ {} {} {}",
            brick.from.x, brick.from.y, brick.from.z, brick.to.x, brick.to.y, brick.to.z
        )
    }

    let mut brick_supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut brick_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..bricks.len() {
        for j in 0..i {
            if bricks[i].overlaps(&bricks[j]) && bricks[j].roof() + 1 == bricks[i].floor() {
                brick_supports.entry(j).or_default().insert(i);
                brick_supported_by.entry(i).or_default().insert(j);
            }
        }
    }

    let mut silver = 0;
    for i in 0..bricks.len() {
        if brick_supports
            .entry(i)
            .or_default()
            .iter()
            .all(|d| brick_supported_by[d].len() != 1)
        {
            silver += 1;
        }
    }

    println!("silver: {}", silver);

    let mut gold = 0;
    for i in 0..bricks.len() {
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(i);
        let mut falling: HashSet<usize> = HashSet::new();
        falling.insert(i);
        while let Some(current) = queue.pop_front() {
            for other in brick_supports
                .get(&current)
                .unwrap_or(&HashSet::new())
                .iter()
                .filter(|d| {
                    brick_supported_by[d].len() > 0
                        && brick_supported_by[d].iter().all(|x| falling.contains(x))
                })
                .collect::<Vec<_>>()
            {
                queue.push_back(*other);
                falling.insert(*other);
            }
        }
        gold += falling.len() - 1;
    }

    println!("gold: {}", gold);
}
