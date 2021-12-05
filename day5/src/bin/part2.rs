use std::collections::HashMap;

use day5::*;

use aoc::*;

fn main() {
    let vents = parser::lines::<Vent>()
        .flat_map(|vent| {
            if vent.orthogonal() {
                Box::new(vent.range) as Box<dyn Iterator<Item = Coord<usize>>>
            } else {
                let Range { start, end, .. } = vent.range;
                let x = if start.x > end.x {
                    Box::new((end.x..=start.x).rev()) as Box<dyn Iterator<Item = usize>>
                } else {
                    Box::new(start.x..=end.x) as Box<dyn Iterator<Item = usize>>
                };
                let y = if start.y > end.y {
                    Box::new((end.y..=start.y).rev()) as Box<dyn Iterator<Item = usize>>
                } else {
                    Box::new(start.y..=end.y) as Box<dyn Iterator<Item = usize>>
                };
                Box::new(x.zip(y).map(|tuple| tuple.into()))
                    as Box<dyn Iterator<Item = Coord<usize>>>
            }
        })
        .fold(HashMap::new(), |mut map, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        });

    answer!(
        "There is {} points that do at least two overlaps.",
        vents.values().filter(|v| **v > 1).count()
    );
}
