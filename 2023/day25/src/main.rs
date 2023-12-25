use rayon::prelude::*;
use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use itertools::Itertools;
use petgraph::{
    algo::connected_components,
    graph::UnGraph,
    visit::{depth_first_search, DfsEvent},
};

fn main() {
    let mut g = UnGraph::<&str, ()>::default();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut nodes = HashMap::new();
    for line in input.split_terminator('\n') {
        let (node, edges) = line.split_once(':').unwrap();
        let node = node.trim();
        let node = *nodes.entry(node).or_insert_with(|| g.add_node(node));
        for edge in edges.split(' ').map(|x| x.trim()).filter(|x| !x.is_empty()) {
            let edge = *nodes.entry(edge).or_insert_with(|| g.add_node(edge));
            g.add_edge(node, edge, ());
        }
    }

    let mut silver = 0;
    eprintln!("{} {}", g.node_count(), g.edge_count());
    let result = g
        .edge_indices()
        .tuple_combinations()
        .par_bridge()
        .map(|(e1, e2, e3)| {
            let mut gprime = g.clone();
            let (e1n1, _) = gprime.edge_endpoints(e1).unwrap();
            gprime.remove_edge(e1);
            gprime.remove_edge(e2);
            gprime.remove_edge(e3);
            if connected_components(&gprime) == 2 {
                let mut result = 0;
                depth_first_search(&gprime, Some(e1n1), |event| {
                    if let DfsEvent::Discover(_, _) = event {
                        result += 1;
                    }
                });
                return result;
            }
            return 0;
        })
        .find_any(|x| x > &0);
    silver = result.unwrap();

    silver *= g.node_count() - silver;
    println!("silver: {}", silver);

    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
