#![feature(let_chains)]

use crate::ModuleType::Conjunction;
use crate::Pulse::{High, Low};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::io::{stdin, Read};
use num::integer::lcm;
use ModuleType::FlipFlop;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Module<'a> {
    destinations: Vec<&'a str>,
    module_type: ModuleType<'a>,
}

#[derive(Clone, Eq, PartialEq)]
struct ConjunctionData<'a> {
    source_memory: HashMap<&'a str, bool>,
}

impl<'a> Hash for ConjunctionData<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.source_memory.iter().collect();
        pairs.sort_by_key(|x| x.0);
        Hash::hash(&pairs, state);
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjunction(ConjunctionData<'a>),
    None,
}

fn print_dot(modules: &HashMap<&str, Module>) {
    eprintln!("digraph {{");
    for (name, module) in modules {
        for modi in &module.destinations {
            eprintln!("{} -> {}", name, modi);
        }
        if let ModuleType::None = module.module_type {
            eprintln!(
                "{} [shape={}]",
                name,
                match &module.module_type {
                    FlipFlop(_) => "diamond",
                    Conjunction(_) => "box",
                    _ => "",
                }
            );
        }
    }
    eprintln!("}}");
}

#[derive(Eq, PartialEq, Clone)]
enum Pulse {
    Low,
    High,
}

fn silver<'a>(modules: &'a mut HashMap<&'a str, Module<'a>>) -> usize {
    let mut low = 0;
    let mut high = 0;
    let mut i = 0;
    // nd hf sb ds
    let mut last_conjs = HashMap::new();
    loop {
        i += 1;
        let mut queue = VecDeque::new();
        queue.push_back((Low, "broadcaster", "button"));
        while let Some((pulse, current, from)) = queue.pop_front() {
            if ["nd", "hf", "sb", "ds"].contains(&from) && matches!(pulse, High) && !last_conjs.contains_key(from) {
                last_conjs.insert(from, i);
            }
            if last_conjs.len() >= 4 {
                return last_conjs.values().copied().reduce(lcm).unwrap();
            }
            if current == "rx" && matches!(pulse, Low) {
                return i;
            }
            match pulse {
                Low => low += 1,
                High => high += 1,
            }
            let current_module = match modules.get_mut(current) {
                Some(cur) => cur,
                None => {
                    continue;
                }
            };

            if let FlipFlop(val) = current_module.module_type {
                if let High = pulse {
                    continue;
                }
                current_module.module_type = FlipFlop(!val);
                for destination_module in &current_module.destinations {
                    queue.push_back((if val { Low } else { High }, destination_module, current))
                }
            } else if let Conjunction(data) = &mut current_module.module_type {
                let current_memory = data.source_memory.get_mut(from).unwrap();
                *current_memory = match pulse {
                    Low => false,
                    High => true,
                };
                let sent_pulse = if data.source_memory.values().all(|x| *x) {
                    Low
                } else {
                    High
                };
                for destination_module in &current_module.destinations {
                    queue.push_back((sent_pulse.clone(), destination_module, current))
                }
            } else {
                for destination_module in &current_module.destinations {
                    queue.push_back((pulse.clone(), destination_module, current));
                }
            }
        }
    }
    low * high
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
        .filter(|(k, m)| matches!(m.module_type, Conjunction(_)))
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
    // print_dot(&modules);
    println!("silver: {}", silver(&mut modules));
    // reset states
    // for module in modules.values_mut() {
    //     match &mut module.module_type {
    //         FlipFlop(val) => {
    //             *val = false;
    //         }
    //         Conjunction(data) => {
    //             for value in data.source_memory.values_mut() {
    //                 *value = false;
    //             }
    //         }
    //         ModuleType::None => {}
    //     }
    // }
}
