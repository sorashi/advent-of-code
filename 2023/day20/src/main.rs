use crate::ModuleType::Conjunction;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, Read};
use ModuleType::FlipFlop;

#[derive(Clone, Eq, PartialEq)]
struct Module<'a> {
    destinations: Vec<&'a str>,
    module_type: ModuleType<'a>,
}

#[derive(Clone, Eq, PartialEq)]
struct ConjunctionData<'a> {
    source_memory: HashMap<&'a str, bool>,
}

#[derive(Clone, Eq, PartialEq)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjunction(ConjunctionData<'a>),
    None,
}

type Modules<'a> = HashMap<&'a str, Module<'a>>;

fn press_button<F>(modules: &mut Modules, visitor: &mut F) -> bool
where
    F: FnMut((bool, &str, &str)) -> bool,
{
    let mut queue = VecDeque::new();
    queue.push_back((false, "broadcaster", "button"));
    while let Some((pulse, current, from)) = queue.pop_front() {
        if visitor((pulse, current, from)) {
            return true;
        }

        let current_module = match modules.get_mut(current) {
            Some(cur) => cur,
            None => {
                continue;
            }
        };

        if let FlipFlop(val) = current_module.module_type {
            if pulse {
                continue;
            }
            current_module.module_type = FlipFlop(!val);
            for destination_module in &current_module.destinations {
                queue.push_back((!val, destination_module, current))
            }
        } else if let Conjunction(data) = &mut current_module.module_type {
            let current_memory = data.source_memory.get_mut(from).unwrap();
            *current_memory = pulse;
            let sent_pulse = !data.source_memory.values().all(|x| *x);
            for destination_module in &current_module.destinations {
                queue.push_back((sent_pulse, destination_module, current))
            }
        } else {
            for destination_module in &current_module.destinations {
                queue.push_back((pulse, destination_module, current));
            }
        }
    }
    false
}

fn silver(modules: &mut Modules) -> usize {
    let mut low = 0;
    let mut high = 0;
    for _ in 1..=1000 {
        press_button(modules, &mut |(pulse, _, _)| {
            if pulse {
                high += 1
            } else {
                low += 1
            }
            false
        });
    }
    low * high
}

fn gold(modules: &mut Modules) -> usize {
    let mut prev_conj = HashMap::new();
    let mut i = 0usize;
    loop {
        i += 1;
        let mut closure = |(pulse, _, from): (bool, &str, &str)| {
            if ["nd", "hf", "sb", "ds"].contains(&from) && !prev_conj.contains_key(from) && pulse {
                prev_conj.insert(from.to_string(), i);
            }
            prev_conj.len() >= 4
        };
        if press_button(modules, &mut closure) {
            break;
        }
    }
    prev_conj.values().copied().reduce(lcm).unwrap()
}

fn main() {
    let mut modules: HashMap<&str, Module> = HashMap::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    for line in input.split_terminator('\n') {
        let (mut from, to) = line.split_once("->").unwrap();
        let destinations = to.split(',').map(|s| s.trim()).collect();
        from = from.trim();
        let mtype = match from.as_bytes()[0] {
            b'%' => FlipFlop(false),
            b'&' => Conjunction(ConjunctionData {
                source_memory: HashMap::new(),
            }),
            _ => ModuleType::None,
        };
        if !matches!(mtype, ModuleType::None) {
            from = &from[1..];
        }
        let module = Module {
            destinations,
            module_type: mtype,
        };
        modules.insert(from, module);
    }
    let modules_copy = modules.clone();
    for (con_name, con_module) in modules
        .iter_mut()
        .filter(|(_, m)| matches!(m.module_type, Conjunction(_)))
    {
        let sources = modules_copy
            .iter()
            .filter(|(_, mo)| mo.destinations.contains(con_name))
            .map(|x| (*x.0, false))
            .collect();
        if let Conjunction(data) = &mut con_module.module_type {
            data.source_memory = sources;
        }
    }
    println!("silver: {}", silver(&mut modules));

    // reset state
    for module in modules.values_mut() {
        match &mut module.module_type {
            FlipFlop(val) => {
                *val = false;
            }
            Conjunction(data) => {
                for value in data.source_memory.values_mut() {
                    *value = false;
                }
            }
            ModuleType::None => {}
        }
    }

    println!("gold: {}", gold(&mut modules));
}
