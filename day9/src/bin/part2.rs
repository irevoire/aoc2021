use std::{cmp::Reverse, collections::HashSet};

use aoc::*;
use Direction::*;

fn main() {
    let grid = parser::lines::<String>()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let grid = Grid::from(grid);

    let mut low_points: Vec<usize> = Coord::at(0, 0)
        .to(Coord::at(grid.width() - 1, grid.height() - 1))
        .unwrap()
        .filter_map(|coord| {
            [East, South, North, West]
                .into_iter()
                .filter_map(|dir| coord.checked_add(dir).and_then(|coord| grid.get(coord)))
                .all(|adj| adj > &grid[coord])
                .then(|| coord)
        })
        .map(|low_point| basin_size(&grid, low_point, &mut HashSet::new()))
        .collect();

    low_points.sort_by_key(|p| Reverse(*p));

    answer!(
        "The sizes of the three largest basins multiplied together is {}.",
        low_points[0] * low_points[1] * low_points[2]
    );
}

fn basin_size(
    grid: &Grid<usize>,
    point: Coord<usize>,
    explored: &mut HashSet<Coord<usize>>,
) -> usize {
    if explored.contains(&point) {
        return 0;
    }
    explored.insert(point);
    match grid.get(point) {
        Some(9) => 0,
        Some(_) => {
            1 + point
                .checked_add(South)
                .map_or(0, |point| basin_size(grid, point, explored))
                + point
                    .checked_add(North)
                    .map_or(0, |point| basin_size(grid, point, explored))
                + point
                    .checked_add(East)
                    .map_or(0, |point| basin_size(grid, point, explored))
                + point
                    .checked_add(West)
                    .map_or(0, |point| basin_size(grid, point, explored))
        }
        None => 0,
    }
}
