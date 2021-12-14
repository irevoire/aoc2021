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

    let polymer = polymerize(
        Box::new(polymer.clone().into_iter().tuple_windows()),
        &transformations,
        40,
        &mut HashMap::new(),
    );

    let min = polymer.values().min().unwrap();
    let max = polymer.values().max().unwrap();

    answer!("If you take the quantity of the most common element and subtract the quantity of the least common element you get {}.", max - min);
}

pub fn polymerize<'a>(
    polymer: Box<dyn Iterator<Item = (char, char)> + 'a>,
    transformations: &'a HashMap<(char, char), char>,
    depth: usize,
    cache: &mut HashMap<(usize, (char, char)), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    static mut FIRST: bool = true;
    if depth == 0 {
        polymer.fold(HashMap::new(), |mut map, (a, b)| {
            if unsafe { FIRST } {
                *map.entry(a).or_insert(0) += 1;
                unsafe { FIRST = false };
            }
            *map.entry(b).or_insert(0) += 1;
            map
        })
    } else {
        polymer
            .map(|w| {
                if let Some(res) = cache.get(&(depth, w)) {
                    res.clone()
                } else {
                    let insert = transformations[&w];
                    let res = polymerize(
                        Box::new([(w.0, insert), (insert, w.1)].into_iter()),
                        transformations,
                        depth - 1,
                        cache,
                    );
                    cache.insert((depth, w), res.clone());
                    res
                }
            })
            .fold(HashMap::new(), |mut res, el| {
                for (key, value) in el.into_iter() {
                    *res.entry(key).or_insert(0) += value;
                }
                res
            })
    }
}
