use std::collections::HashMap;

use aoc::*;

fn main() {
    let input = parser::input::<String>();
    let (polymer, transformations) = input.split_once("\n\n").unwrap();
    let mut polymer = polymer.to_string();

    let transformations: HashMap<&str, char> = transformations
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(key, value)| (key, value.chars().next().unwrap()))
        .collect();

    for _ in 0..10 {
        polymer = polymerize(polymer, &transformations);
    }

    let polymer = polymer
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

    let min = polymer.values().min().unwrap();
    let max = polymer.values().max().unwrap();

    answer!("If you take the quantity of the most common element and subtract the quantity of the least common element you get {}.", max - min);
}

pub fn polymerize(polymer: String, transformations: &HashMap<&str, char>) -> String {
    let polymer: Vec<char> = polymer.chars().collect();

    polymer
        .windows(2)
        .map(|w| {
            [
                w[0],
                *(transformations
                    .get(w.iter().collect::<String>().as_str())
                    .unwrap()),
            ]
            .iter()
            .collect::<String>()
        })
        .chain(std::iter::once(polymer.last().unwrap().to_string()))
        .join("")
}
