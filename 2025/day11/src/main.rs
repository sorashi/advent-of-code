use std::{collections::HashMap, io::stdin};

fn count_paths_between(
    a: &str,
    b: &str,
    node_map: &HashMap<String, usize>,
    adj: &[Vec<usize>],
) -> usize {
    let start = node_map[a];
    let end = node_map[b];
    let mut counts = vec![usize::MAX; adj.len()];
    fn visit(node: usize, end: usize, counts: &mut Vec<usize>, adj: &[Vec<usize>]) -> usize {
        if counts[node] != usize::MAX {
            return counts[node];
        }
        if node == end {
            counts[end] = 1;
            return 1;
        }
        let mut count = 0;
        for &i in &adj[node] {
            count += visit(i, end, counts, adj);
        }
        counts[node] = count;
        count
    }
    visit(start, end, &mut counts, adj)
}

fn main() {
    let mut node_map = HashMap::<String, usize>::new();
    let mut next_id = 0usize;
    let mut adj = Vec::<Vec<usize>>::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let (node, out) = line.split_once(": ").unwrap();
        let node_id = *node_map.entry(node.to_string()).or_insert_with(|| {
            let id = next_id;
            adj.push(vec![]);
            next_id += 1;
            id
        });
        for out in out.split(' ') {
            let out_id = node_map.entry(out.to_string()).or_insert_with(|| {
                let id = next_id;
                adj.push(vec![]);
                next_id += 1;
                id
            });
            adj[node_id].push(*out_id);
        }
    }
    println!(
        "silver: {}",
        count_paths_between("you", "out", &node_map, &adj)
    );
    let a = count_paths_between("svr", "fft", &node_map, &adj);
    let b = count_paths_between("fft", "dac", &node_map, &adj);
    let c = count_paths_between("dac", "out", &node_map, &adj);
    println!("gold: {}", a * b * c);
}
