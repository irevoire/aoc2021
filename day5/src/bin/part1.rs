use day5::*;
use std::{collections::HashMap, str::FromStr};

use aoc::*;

fn main() {
    let vents = parser::lines::<Vent>()
        .filter(|vent| vent.orthogonal())
        .flat_map(|vent| vent.range)
        .fold(HashMap::new(), |mut map, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        });
    answer!(
        "There is {} points that do at least two overlaps.",
        vents.values().filter(|v| **v > 1).count()
    );
}
