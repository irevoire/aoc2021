use std::collections::HashSet;

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

    println!("{}", grid);

    let (width, height) = (grid.width(), grid.height());

    let base = Coord::at(0, 0);
    let end = Coord::at(width - 1, height - 1);

    let risk = best_route(
        base,
        &end,
        HashSet::new(),
        &grid,
        0,
        &mut (width * height * 5),
    );

    println!("result: {}", risk - grid[base]);
}

pub fn best_route(
    start: Coord<usize>,
    end: &Coord<usize>,
    mut already_explored: HashSet<Coord<usize>>,
    grid: &Grid<usize>,
    total: usize,
    min: &mut usize,
) -> usize {
    const DIRECTIONS: &[Direction] = &[
        Direction::South,
        Direction::East,
        // Direction::West,
        // Direction::North,
    ];
    if already_explored.contains(&start) {
        return usize::MAX;
    }
    let score = total + grid[start];
    if start == *end {
        // dbg!(*min, score);
        *min = std::cmp::min(*min, score);
        // println!("{}", total + grid[start]);
        return *min;
    }

    already_explored.insert(start);

    if score >= *min {
        return score;
    }

    let mut vec = DIRECTIONS
        .iter()
        // ensure if the coordinates are positive
        .filter_map(|dir| start.checked_add(*dir))
        // ensure the coordinates fit in the grid
        .filter(|coord| grid.get(*coord).is_some())
        .collect::<Vec<_>>();
    vec.sort_by_key(|coord| grid[coord]);

    vec.into_iter()
        .map(|coord| best_route(coord, end, already_explored.clone(), grid, score, min))
        .min()
        .unwrap_or(usize::MAX)
}
