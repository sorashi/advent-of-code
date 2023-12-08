use num::integer::lcm;
use hashbrown::HashMap;
use std::io::{stdin, Read};

type N = u64;

struct Crossroad<'a> {
    left: &'a str,
    right: &'a str,
}

fn get_step_count(
    from: &str,
    ending_predicate: impl Fn(&str) -> bool,
    map: &HashMap<&str, Crossroad>,
    instructions: &str,
) -> N {
    let mut current = from;
    let mut steps: N = 0;
    for instruction in instructions.as_bytes().iter().map(|x| *x as char).cycle() {
        let crossroad = map.get(current).unwrap();
        current = match instruction {
            'L' => &crossroad.left,
            'R' => &crossroad.right,
            _ => panic!(),
        };
        steps += 1;
        if ending_predicate(current) {
            break;
        }
    }
    steps
}

fn main() {
    let mut inp = String::new();
    stdin().read_to_string(&mut inp).unwrap();
    let (instructions, rest) = inp.split_once("\n\n").unwrap();
    let instructions = instructions.trim();
    let mut map = HashMap::new();
    for l in rest.split_terminator('\n') {
        map.insert(
            &l[..3],
            Crossroad {
                left: &l[7..10],
                right: &l[12..15],
            },
        );
    }

    let silver = get_step_count("AAA", |x| x == "ZZZ", &map, &instructions);
    let gold = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| get_step_count(x, |x| x.ends_with('Z'), &map, &instructions))
        .reduce(lcm)
        .unwrap();
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
