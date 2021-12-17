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

    let base = Coord::at(0, 0);
    let mut map = [(base, grid[base])].into_iter().collect();

    let risk = compute_risk_level(&grid, &mut vec![base], &mut map);

    answer!(
        "The lowest total risk of any path from the top left to the bottom right is: {}.",
        risk - grid[base]
    );
}

fn compute_risk_level(
    grid: &Grid<usize>,
    to_explore: &mut Vec<Coord>,
    explored_cell: &mut HashMap<Coord, usize>,
) -> usize {
    let current_coord = to_explore.pop().unwrap();
    let current_risk = explored_cell[&current_coord];

    if current_coord == (grid.width() - 1, grid.height() - 1) {
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
    .filter(|coord| grid.get(*coord).is_some())
    .filter(|coord| !explored_cell.contains_key(&coord))
    .collect::<Vec<Coord>>();

    for coord in iter {
        explored_cell.insert(coord, current_risk + grid[coord]);
        to_explore.binary_insert_by_key(coord, |coord| Reverse(explored_cell[coord]));
    }

    // recursive tail
    compute_risk_level(grid, to_explore, explored_cell)
}
