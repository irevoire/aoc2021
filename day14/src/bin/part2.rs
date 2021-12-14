use std::collections::HashMap;

use aoc::*;

fn main() {
    let input = parser::input::<String>();
    let (polymer, transformations) = input.split_once("\n\n").unwrap();
    let polymer = polymer.chars().collect::<Vec<char>>();

    let transformations: HashMap<(char, char), char> = transformations
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(key, value)| {
            let key: Vec<char> = key.chars().collect();
            let key = (key[0], key[1]);
            (key, value.chars().next().unwrap())
        })
        .collect();

    /*
    for depth in 1..5 {
        let polymer = polymerize(
            Box::new(polymer.clone().into_iter().tuple_windows()),
            &transformations,
            depth,
        )
        .collect::<String>();
        println!("After step {}: {}", depth, polymer);
    }
    */

    for step in 1..25 {
        let polymer = polymerize(
            Box::new(polymer.clone().into_iter().tuple_windows()),
            &transformations,
            step,
        )
        // .progress_count(2192039569602)
        .fold(HashMap::<char, usize>::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let min = polymer.values().min().unwrap();
        let max = polymer.values().max().unwrap();
        let length = polymer.values().sum::<usize>();
        dbg!(length);

        println!("step {:2}:\t{}", step, max - min);
        // answer!("If you take the quantity of the most common element and subtract the quantity of the least common element you get {}.", max - min);
    }
}

pub fn polymerize<'a>(
    polymer: Box<dyn Iterator<Item = (char, char)> + 'a>,
    transformations: &'a HashMap<(char, char), char>,
    depth: usize,
) -> Box<dyn Iterator<Item = char> + 'a> {
    // let cache: HashMap<(char, char), HashMap<char, usize>> = HashMap::new();
    let iter = polymer.flat_map(|w| [w.0, *(transformations.get(&w).unwrap())]);
    if depth == 1 {
        Box::new(iter) as Box<dyn Iterator<Item = char> + 'a>
    } else {
        Box::new(polymerize(
            Box::new(iter.tuple_windows()),
            transformations,
            depth - 1,
        )) as Box<dyn Iterator<Item = char> + 'a>
    }
}
