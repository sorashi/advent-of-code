use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};

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
            return (
                name,
                Workflow {
                    steps: workflow_steps,
                    fallback: step,
                },
            );
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
        } else {
            0
        }
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
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.from, self.to))
    }
}

type ParameterIntervals = HashMap<u8, Interval>;

fn get_interval_combos(intervals: &ParameterIntervals) -> usize {
    intervals.iter().map(|x| x.1.count()).product()
}

fn get_possible_combination_count(
    workflows: &HashMap<&str, Workflow>,
    current_intervals: ParameterIntervals,
    current_workflow: &str,
) -> usize {
    if current_workflow == "A" {
        return get_interval_combos(&current_intervals);
    } else if current_workflow == "R" {
        return 0;
    }
    let workflow = &workflows[current_workflow];
    let mut intervals = current_intervals.clone();
    let mut result = 0;
    let mut out_of_intervals = false;
    for step in &workflow.steps {
        let (branch_intervals, parameter, target) = match step {
            WorkflowStep::GreaterThan(parameter, value, target) => {
                let mut branch_intervals = intervals.clone();
                branch_intervals
                    .get_mut(parameter)
                    .unwrap()
                    .restrict_gt(*value);
                intervals.get_mut(parameter).unwrap().restrict_lte(*value);
                (branch_intervals, parameter, target)
            }
            WorkflowStep::LessThan(parameter, value, target) => {
                let mut branch_intervals = intervals.clone();
                branch_intervals
                    .get_mut(parameter)
                    .unwrap()
                    .restrict_lt(*value);
                intervals.get_mut(parameter).unwrap().restrict_gte(*value);
                (branch_intervals, parameter, target)
            }
        };

        if branch_intervals[parameter].any() {
            result += get_possible_combination_count(workflows, branch_intervals, target);
        }
        if get_interval_combos(&intervals) == 0 {
            out_of_intervals = true;
            break;
        }
    }
    if !out_of_intervals {
        result += get_possible_combination_count(workflows, intervals, workflow.fallback);
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

    let from: usize = 1;
    let to: usize = 4000;
    let gold = get_possible_combination_count(
        &workflows_map,
        "xmas"
            .as_bytes()
            .iter()
            .map(|b| (*b, Interval { from, to }))
            .collect(),
        "in",
    );

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
