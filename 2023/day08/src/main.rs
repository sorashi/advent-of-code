use std::collections::HashMap;
use std::io::stdin;
use regex::Regex;
use num::integer::lcm;

type N = u128;

struct Crossroad {
    left: String,
    right: String,
}

fn get_step_count(from: &str, ending_predicate: impl Fn(&str) -> bool, map: &HashMap<String, Crossroad>, instructions: &str) -> N {
    let mut current = from;
    let mut steps: N = 0;
    for instruction in instructions.as_bytes().iter().map(|x| *x as char).cycle() {
        let crossroad = map.get(current).unwrap();
        current = match instruction {
            'L' => &crossroad.left,
            'R' => &crossroad.right,
            _ => panic!()
        };
        steps += 1;
        if ending_predicate(current) {
            break;
        }
    }
    steps
}

fn main() {
    let mut instructions = String::new();
    stdin().read_line(&mut instructions).unwrap();
    instructions = instructions.trim().to_string();
    stdin().read_line(&mut String::new()).unwrap();
    let re = Regex::new(r"^(?P<from>\w+) = \((?P<left>\w+), (?P<right>\w+)\)$").unwrap();
    let mut map = HashMap::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        let from = cap["from"].to_string();
        let left = cap["left"].to_string();
        let right = cap["right"].to_string();
        map.insert(from, Crossroad { left, right });
    }
    let silver = get_step_count("AAA", |x| x == "ZZZ", &map, &instructions);
    let gold = map.keys().filter(|x| x.ends_with('A')).map(|x| get_step_count(x, |x| x.ends_with('Z'), &map, &instructions)).reduce(|a, c| lcm(a, c)).unwrap();
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
