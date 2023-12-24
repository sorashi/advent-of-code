use itertools::Itertools;
use num::{BigInt, BigRational, ToPrimitive, Zero};
use std::{
    fmt::Display,
    io::{stdin, Read},
    ops::{Add, Mul},
};
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
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

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let prx = Int::new_const(&ctx, "prx");
    let pry = Int::new_const(&ctx, "pry");
    let prz = Int::new_const(&ctx, "prz");
    let vrx = Int::new_const(&ctx, "vrx");
    let vry = Int::new_const(&ctx, "vry");
    let vrz = Int::new_const(&ctx, "vrz");

    for hailstone in hailstones {
        let px = Int::from_big_int(&ctx, &hailstone.pos.x.to_integer());
        let py = Int::from_big_int(&ctx, &hailstone.pos.y.to_integer());
        let pz = Int::from_big_int(&ctx, &hailstone.pos.z.to_integer());
        let vx = Int::from_big_int(&ctx, &hailstone.vel.x.to_integer());
        let vy = Int::from_big_int(&ctx, &hailstone.vel.y.to_integer());
        let vz = Int::from_big_int(&ctx, &hailstone.vel.z.to_integer());

        let eq1 = ((&prx - px) * (&vy - &vry))._eq(&((&pry - &py) * (vx - &vrx)));
        let eq2 = ((&pry - py) * (vz - &vrz))._eq(&((&prz - pz) * (vy - &vry)));
        solver.assert(&eq1);
        solver.assert(&eq2);
    }

    let result = solver.check();
    assert!(matches!(result, z3::SatResult::Sat));
    let model = solver.get_model().unwrap();
    let prx = model.get_const_interp(&prx).unwrap();
    let pry = model.get_const_interp(&pry).unwrap();
    let prz = model.get_const_interp(&prz).unwrap();
    let prx: BigInt = prx.to_string().parse().unwrap();
    let pry: BigInt = pry.to_string().parse().unwrap();
    let prz: BigInt = prz.to_string().parse().unwrap();
    let gold = prx + pry + prz;

    println!("gold: {}", gold);
}
