use std::collections::HashMap;

use day5::*;

use aoc::*;

fn main() {
    let vents: Vec<_> = parser::lines::<Vent>()
        .flat_map(|vent| vent.range)
        .collect();

    let vents = vents
        .par_iter()
        .progress_count(vents.len() as u64)
        .fold(
            || HashMap::new(),
            |mut map, point| {
                *map.entry(point).or_insert(0) += 1;
                map
            },
        )
        .reduce_with(|mut acc, map| {
            map.into_iter()
                .for_each(|(key, value)| *acc.entry(key).or_insert(0) += value);
            acc
        })
        .unwrap();
    answer!(
        "There is {} points that do at least two overlaps.",
        vents.values().filter(|v| **v > 1).count()
    );
}
