use std::{cmp::Reverse, collections::HashMap};

use aoc::*;

fn main() {
    let input = parser::lines::<String>()
        .map(|line| {
            line.trim()
                .split("")
                .filter_map(|i| i.trim().parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let grid = Grid::from(input);

    let risk = compute_risk_level(&grid);

    answer!(
        "The lowest total risk of any path from the top left to the bottom right is: {}.",
        risk - grid[(0, 0)]
    );
}

fn compute_risk_level(grid: &Grid<usize>) -> usize {
    let base = Coord::at(0, 0);
    let mut to_explore: Vec<Coord> = vec![base];
    let mut explored_cell: HashMap<Coord, usize> = [(base, grid[base])].into_iter().collect();

    loop {
        let current_coord = to_explore.pop().unwrap();
        let current_risk = explored_cell[&current_coord];

        if current_coord == (grid.width() * 5 - 1, grid.height() * 5 - 1) {
            return current_risk;
        }

        let iter = [
            Direction::North,
            Direction::East,
            Direction::West,
            Direction::South,
        ]
        .into_iter()
        .filter_map(|direction| current_coord.checked_add(direction))
        .map(|coord| {
            let mut patched_coord = coord.clone();
            patched_coord.x = coord.x % grid.width();
            patched_coord.y = coord.y % grid.height();
            (coord, patched_coord)
        })
        .filter(|(_, patched_coord)| grid.get(*patched_coord).is_some())
        .filter(|(coord, _)| !explored_cell.contains_key(&coord))
        .collect::<Vec<(Coord, Coord)>>();

        for (coord, patched_coord) in iter {
            let mut risk = grid[patched_coord] + coord.x / grid.width() + coord.y / grid.height();
            if risk >= 10 {
                risk = risk % 10 + 1;
            }
            explored_cell.insert(coord, current_risk + risk);
            to_explore.binary_insert_by_key(coord, |coord| Reverse(explored_cell[coord]));
        }
    }
}
