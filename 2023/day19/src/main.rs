use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};
use std::panic::panic_any;

struct Workflow<'a> {
    steps: Vec<WorkflowStep<'a>>,
    fallback: &'a str,
}

enum WorkflowStep<'a> {
    GreaterThan(u8, usize, &'a str),
    LessThan(u8, usize, &'a str),
}

fn parse_workflow(s: &str) -> (&str, Workflow) {
    let first_brace = s.find('{').unwrap();
    let name = &s[..first_brace];
    let steps = &s[first_brace + 1..s.len() - 1];
    let steps = steps.split(',');
    let mut workflow_steps = vec![];
    for step in steps {
        let split = step.split_once(':');
        if let Some((step, target)) = split {
            let relation = &step[1..2];
            let parameter = step.as_bytes()[0];
            let value: usize = step[2..].parse().unwrap();
            let step = match relation {
                ">" => WorkflowStep::GreaterThan(parameter, value, target),
                "<" => WorkflowStep::LessThan(parameter, value, target),
                _ => panic!(),
            };
            workflow_steps.push(step);
        } else {
            return (name, Workflow { steps: workflow_steps, fallback: step });
        }
    }
    panic!()
}

fn parse_rating(s: &str) -> HashMap<u8, usize> {
    let s = &s[1..s.len() - 1];
    let mut map = HashMap::new();
    for rating in s.split(',') {
        let (parameter, value) = rating.split_once('=').unwrap();
        map.insert(parameter.as_bytes()[0], value.parse().unwrap());
    }
    map
}

fn is_accepted(
    rating: &HashMap<u8, usize>,
    workflows: &HashMap<&str, Workflow>,
    current_workflow: &str,
) -> bool {
    if current_workflow == "A" {
        return true;
    } else if current_workflow == "R" {
        return false;
    }
    let workflow = &workflows[current_workflow];
    for step in &workflow.steps {
        match step {
            WorkflowStep::GreaterThan(parameter, value, target) => {
                if rating[parameter] > *value {
                    return is_accepted(rating, workflows, target);
                }
            }
            WorkflowStep::LessThan(parameter, value, target) => {
                if rating[parameter] < *value {
                    return is_accepted(rating, workflows, target);
                }
            }
        }
    }
    is_accepted(rating, workflows, workflow.fallback)
}

/// Inclusive
#[derive(Clone)]
struct Interval {
    from: usize,
    to: usize,
}

impl Interval {
    fn count(&self) -> usize {
        if self.any() {
            self.to + 1 - self.from
        } else { 0 }
    }
    fn any(&self) -> bool {
        self.to >= self.from
    }
    fn restrict_gt(&mut self, value: usize) {
        self.from = self.from.max(value + 1);
    }
    fn restrict_gte(&mut self, value: usize) {
        self.from = self.from.max(value);
    }
    fn restrict_lt(&mut self, value: usize) {
        self.to = self.to.min(value - 1);
    }
    fn restrict_lte(&mut self, value: usize) {
        self.to = self.to.min(value);
    }
    fn intersect(&self, other: &Self) -> Self {
        Self {
            from: max(self.from, other.from),
            to: min(self.to, other.to),
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.from, self.to))
    }
}


type ParameterIntervals = HashMap<u8, Interval>;

fn print_intervals(inters: &ParameterIntervals) {
    for k in [b'x', b'm', b'a', b's'] {
        eprint!("{}: {}, ", k as char, inters[&k]);
    }
    eprintln!();
}

fn get_combos(
    workflows: &HashMap<&str, Workflow>,
    current_combos: ParameterIntervals,
    current_workflow: &str,
) -> usize {
    if current_workflow == "A" {
        return current_combos.iter().map(|x| x.1.count()).product();
    } else if current_workflow == "R" {
        return 0;
    }
    let workflow = &workflows[current_workflow];
    let mut combos = current_combos.clone();
    let mut result = 0;
    for step in &workflow.steps {
        match step {
            WorkflowStep::GreaterThan(parameter, value, target) => {
                let mut branch_intervals = combos.clone();
                branch_intervals.get_mut(parameter).unwrap().restrict_gt(*value);
                if branch_intervals[&parameter].any() {
                    result += get_combos(workflows, branch_intervals, target);
                }
                combos.get_mut(parameter).unwrap().restrict_lte(*value);
            }
            WorkflowStep::LessThan(parameter, value, target) => {
                let mut branch_intervals = combos.clone();
                branch_intervals.get_mut(parameter).unwrap().restrict_lt(*value);
                if branch_intervals[&parameter].any() {
                    result += get_combos(workflows, branch_intervals, target);
                }
                combos.get_mut(parameter).unwrap().restrict_gte(*value);
            }
        }
        if combos.iter().map(|x|x.1.count()).product::<usize>() == 0 {
            break;
        }
    }
    if combos.iter().map(|x|x.1.count()).product::<usize>() != 0 {
        result += get_combos(workflows, combos, workflow.fallback);
    }
    result
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (workflows, ratings) = input.split_once("\n\n").unwrap();
    let mut workflows_map: HashMap<&str, Workflow> = HashMap::new();
    for workflow in workflows.split_terminator('\n') {
        let (name, workflow) = parse_workflow(workflow);
        workflows_map.insert(name, workflow);
    }

    let mut silver = 0;
    for rating in ratings.split_terminator('\n') {
        let rating = parse_rating(rating);
        if is_accepted(&rating, &workflows_map, "in") {
            silver += rating.values().sum::<usize>();
        }
    }

    // print_gv(&workflows_map);
    let from: usize = 1;
    let to: usize = 4000;
    let gold = get_combos(&workflows_map, HashMap::from([(b'x', Interval { from, to }), (b'm', Interval { from, to }), (b'a', Interval { from, to }), (b's', Interval { from, to })]), "in");

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}