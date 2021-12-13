use aoc::*;
use day12::*;

fn paths(current_name: &str, mut caves: Caves) -> Option<Vec<Vec<String>>> {
    let current_node = caves.get_mut(current_name).unwrap();

    if current_name == "start" && current_node.visited > 0 {
        return None;
    } else if current_node.visited > 1 && current_node.medium {
        return None;
    } else if current_node.visited > 0 && current_node.smol && !current_node.medium {
        return None;
    } else if current_name == "end" {
        return Some(vec![vec!["end".to_string()]]);
    }
    current_node.visited += 1;

    let current_node = caves.get(current_name).unwrap();
    let caves = &caves;

    let mut result = Vec::new();

    for mut res in current_node
        .liaisons
        .iter()
        .filter_map(move |node_name| paths(node_name, caves.clone()))
        .flatten()
    {
        res.insert(0, current_name.to_string());
        result.push(res);
    }

    Some(result)
}

fn main() {
    let caves = parse_caves();

    let mut result = Vec::new();

    for name in caves.keys() {
        let mut caves = caves.clone();

        if name.chars().all(|c| c.is_lowercase()) {
            caves.get_mut(name).unwrap().medium = true;
            result.append(&mut paths("start", caves).unwrap());
        }
    }
    result.sort();
    result.dedup();

    answer!("There is {} paths through this cave system.", result.len());
}
