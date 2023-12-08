use hashbrown::HashMap;
use num::integer::lcm;
use std::io::{stdin, Read};

type N = u64;

const L: u8 = 'L' as u8;
const R: u8 = 'R' as u8;

struct Crossroad {
    left: u32,
    right: u32,
}

fn get_step_count(
    from: u32,
    ending_predicate: impl Fn(u32) -> bool,
    map: &HashMap<u32, Crossroad>,
    instructions: &str,
) -> N {
    let mut current = from;
    let mut steps: N = 0;
    for instruction in instructions.bytes().cycle() {
        let crossroad = map.get(&current).unwrap();
        current = match instruction {
            L => crossroad.left,
            R => crossroad.right,
            _ => panic!(),
        };
        steps += 1;
        if ending_predicate(current) {
            break;
        }
    }
    steps
}

fn encode_node(node: &str) -> u32 {
    node.bytes().fold(0u32, |a, c| (a << 8) | c as u32)
}

fn main() {
    let mut inp = String::new();
    stdin().read_to_string(&mut inp).unwrap();
    let (instructions, rest) = inp.split_once("\n\n").unwrap();
    let instructions = instructions.trim();
    let mut map = HashMap::new();
    for l in rest.split_terminator('\n') {
        map.insert(
            encode_node(&l[..3]),
            Crossroad {
                left: encode_node(&l[7..10]),
                right: encode_node(&l[12..15]),
            },
        );
    }
    let aaa = encode_node("AAA");
    let zzz = encode_node("ZZZ");

    let silver = get_step_count(aaa, |x| x == zzz, &map, &instructions);
    let gold = map
        .keys()
        .filter(|x| **x as u8 == 'A' as u8)
        .map(|x| get_step_count(*x, |x| x as u8 == 'Z' as u8, &map, &instructions))
        .reduce(lcm)
        .unwrap();
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
