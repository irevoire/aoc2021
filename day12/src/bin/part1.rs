use aoc::*;
use day12::*;

fn paths(current_name: &str, mut caves: Caves) -> usize {
    let current_node = caves.get_mut(current_name).unwrap();
    if current_node.visited > 0 && current_node.smol {
        return 0;
    }
    if current_name == "end" {
        return 1;
    }
    current_node.visited += 1;

    let current_node = caves.get(current_name).unwrap();
    let caves = &caves;

    current_node
        .liaisons
        .iter()
        .map(move |node_name| paths(node_name, caves.clone()))
        .sum::<usize>()
}

fn main() {
    let caves = parse_caves();

    answer!(
        "There is {} paths through this cave system.",
        paths("start", caves)
    );
}
