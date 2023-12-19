use std::collections::HashMap;
use std::io::{stdin, Read};

struct Workflow<'a> {
    steps: Vec<WorkflowStep<'a>>,
}

enum WorkflowStep<'a> {
    GreaterThan(u8, usize, WorkflowTarget<'a>),
    LessThan(u8, usize, WorkflowTarget<'a>),
    Target(WorkflowTarget<'a>),
}

enum WorkflowTarget<'a> {
    Part(&'a str),
    Accept,
    Reject,
}

fn parse_target(s: &str) -> WorkflowTarget {
    match s {
        "A" => WorkflowTarget::Accept,
        "R" => WorkflowTarget::Reject,
        _ => WorkflowTarget::Part(s),
    }
}

fn parse_workflow(s: &str) -> (&str, Workflow) {
    let first_brace = s.find('{').unwrap();
    let name = &s[..first_brace];
    let steps = &s[first_brace + 1..s.len() - 1];
    let steps = steps.split(',');
    let mut workflow = Workflow { steps: vec![] };
    for step in steps {
        let split = step.split_once(':');
        if let Some((step, target)) = split {
            let target = parse_target(target);
            let relation = &step[1..2];
            let parameter = step.as_bytes()[0];
            let value: usize = step[2..].parse().unwrap();
            let step = match relation {
                ">" => WorkflowStep::GreaterThan(parameter, value, target),
                "<" => WorkflowStep::LessThan(parameter, value, target),
                _ => panic!(),
            };
            workflow.steps.push(step);
        } else {
            workflow
                .steps
                .push(WorkflowStep::Target(parse_target(step)));
        }
    }
    (name, workflow)
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
    let workflow = &workflows[current_workflow];
    for step in &workflow.steps {
        match step {
            WorkflowStep::GreaterThan(parameter, value, target) => {
                if rating[parameter] > *value {
                    return match target {
                        WorkflowTarget::Part(part) => is_accepted(rating, workflows, part),
                        WorkflowTarget::Accept => true,
                        WorkflowTarget::Reject => false,
                    };
                }
            }
            WorkflowStep::LessThan(parameter, value, target) => {
                if rating[parameter] < *value {
                    return match target {
                        WorkflowTarget::Part(part) => is_accepted(rating, workflows, part),
                        WorkflowTarget::Accept => true,
                        WorkflowTarget::Reject => false,
                    };
                }
            }
            WorkflowStep::Target(target) => {
                return match target {
                    WorkflowTarget::Part(part) => is_accepted(rating, workflows, part),
                    WorkflowTarget::Accept => true,
                    WorkflowTarget::Reject => false,
                };
            }
        }
    }
    panic!()
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
    println!("silver: {}", silver);
}
