use itertools::Itertools;
use num::{BigInt, BigRational, ToPrimitive, Zero};
use std::{
    fmt::Display,
    io::{stdin, Read},
    ops::{Add, Mul},
};

#[derive(Clone, Debug)]
struct Vector3 {
    x: BigRational,
    y: BigRational,
    z: BigRational,
}

impl Vector3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: BigInt::from(x).into(),
            y: BigInt::from(y).into(),
            z: BigInt::from(z).into(),
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.x.to_f64().unwrap(),
            self.y.to_f64().unwrap(),
            self.z.to_f64().unwrap()
        )
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<BigRational> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: BigRational) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
        }
    }
}

impl Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Self::Output {
        self * BigRational::from(BigInt::from(rhs))
    }
}

struct Hailstone {
    pos: Vector3,
    vel: Vector3,
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ pos: {}, vel: {} }}", self.pos, self.vel)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut hailstones = vec![];
    for line in input.split_terminator('\n') {
        let (pos, vel) = line.split_once('@').unwrap();
        let pos: Vec<i64> = pos.split(',').map(|x| x.trim().parse().unwrap()).collect();
        let vel: Vec<i64> = vel.split(',').map(|x| x.trim().parse().unwrap()).collect();
        let pos = Vector3::new(pos[0], pos[1], pos[2]);
        let vel = Vector3::new(vel[0], vel[1], vel[2]);
        hailstones.push(Hailstone { pos, vel });
    }
    let mut silver = 0;
    for (h1, h2) in hailstones.iter().tuple_combinations() {
        let t1_nominator = h2.vel.y.clone() * (h1.pos.x.clone() - h2.pos.x.clone())
            + h2.vel.x.clone() * (h2.pos.y.clone() - h1.pos.y.clone());
        let denominator = h1.vel.y.clone() * h2.vel.x.clone() - h2.vel.y.clone() * h1.vel.x.clone();
        let t2_nominator = h1.vel.y.clone() * (h1.pos.x.clone() - h2.pos.x.clone())
            + h1.vel.x.clone() * (h2.pos.y.clone() - h1.pos.y.clone());
        let zero = BigInt::zero().into();

        if denominator == zero {
            continue;
        }
        let t1 = t1_nominator / denominator.clone();
        let t2 = t2_nominator / denominator.clone();

        if t1 < zero || t2 < zero {
            continue;
        }

        let res1 = h1.pos.clone() + h1.vel.clone() * t1;
        let _res2 = h2.pos.clone() + h2.vel.clone() * t2;

        let l = BigInt::from(200000000000000u64).into();
        let u = BigInt::from(400000000000000u64).into();

        if res1.x >= l && res1.x <= u && res1.y >= l && res1.y <= u {
            silver += 1;
        }
    }
    println!("silver: {}", silver);
}
