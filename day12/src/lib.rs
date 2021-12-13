use aoc::*;
use std::collections::HashMap;

pub type Caves = HashMap<String, Node>;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub smol: bool,
    pub visited: usize,
    pub liaisons: Vec<String>,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            smol: name.chars().all(|c| c.is_lowercase()),
            visited: 0,
            name,
            liaisons: vec![],
        }
    }
}

pub fn parse_caves() -> Caves {
    let mut map: Caves = HashMap::new();

    for line in parser::lines::<String>() {
        let link: Vec<_> = line.split('-').collect();
        let (left_name, right_name) = (link[0], link[1]);

        let left = map
            .entry(left_name.to_string())
            .or_insert(Node::new(left_name.to_string()));
        left.liaisons.push(right_name.to_string());

        let right = map
            .entry(right_name.to_string())
            .or_insert(Node::new(right_name.to_string()));
        right.liaisons.push(left_name.to_string());
    }

    map
}
