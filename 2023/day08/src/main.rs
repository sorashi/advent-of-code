use std::collections::HashMap;
use std::io::stdin;
use regex::Regex;

struct Crossroad {
    left: String,
    right: String,
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
    let mut instruction = 0;
    let mut currents: Vec<_> = map.keys().filter(|x|x.ends_with('A')).map(|x|x.to_string()).collect();
    let mut silver = 0;
    loop {
        instruction %= instructions.len();

        let inst_char = instructions.as_bytes()[instruction] as char;
        for i in 0..currents.len() {
            let current = &currents[i];
            let crossroads = map.get(current).unwrap();
            currents[i] = match inst_char {
                'L' => crossroads.left.to_string(),
                'R' => crossroads.right.to_string(),
                _ => panic!(),
            };
        }
        silver += 1;

        if currents.iter().all(|x|x.ends_with('Z')) {
            break;
        }

        instruction += 1;
    }
    println!("silver: {}", silver);
}
