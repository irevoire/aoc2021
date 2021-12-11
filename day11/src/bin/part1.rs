use std::collections::HashSet;

use aoc::*;
use Direction::*;

fn main() {
    let grid = parser::lines::<String>()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let mut grid = Grid::from(grid);

    let mut flashing = 0;

    for n in 0..100 {
        println!("Step {}", n);
        println!("{}", grid);
        grid.iter_mut().for_each(|cell| *cell += 1);
        step(&mut grid, &mut HashSet::new(), Coord::at(0, 0));
        flashing += grid.iter_mut().filter(|cell| **cell >= 10).count();
        grid.iter_mut()
            .filter(|cell| **cell >= 10)
            .for_each(|cell| *cell = 0);
    }

    println!("{}", flashing);
}

fn step(grid: &mut Grid<usize>, already_done: &mut HashSet<Coord<usize>>, coord: Coord<usize>) {
    if already_done.contains(&coord) {
        return;
    }
    let flashing = grid[coord] == 10;
    already_done.insert(coord);

    let directions = [
        // base coords
        coord.checked_add(West),
        coord.checked_add(South),
        coord.checked_add(North),
        coord.checked_add(East),
        // diagonals
        coord
            .checked_add(North)
            .map(|coord| coord.checked_add(West))
            .flatten(),
        coord
            .checked_add(South)
            .map(|coord| coord.checked_add(West))
            .flatten(),
        coord
            .checked_add(North)
            .map(|coord| coord.checked_add(East))
            .flatten(),
        coord
            .checked_add(South)
            .map(|coord| coord.checked_add(East))
            .flatten(),
    ];

    for coord in directions.into_iter().filter_map(|coord| coord) {
        if let Some(v) = grid.get_mut(coord) {
            if flashing && *v < 10 {
                *v += 1;
                if *v == 10 {
                    already_done.remove(&coord);
                }
            }
            step(grid, already_done, coord);
        }
    }
}
