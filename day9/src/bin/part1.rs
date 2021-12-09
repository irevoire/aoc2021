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

    let risk = Coord::at(0, 0)
        .to(Coord::at(grid.width() - 1, grid.height() - 1))
        .unwrap()
        .filter_map(|coord| {
            [East, South, North, West]
                .into_iter()
                .filter_map(|dir| coord.checked_add(dir).and_then(|coord| grid.get(coord)))
                .all(|adj| adj > &grid[coord])
                .then(|| grid[coord] + 1)
        })
        .sum::<usize>();

    answer!(
        "The sum of the risk levels of all low points on your heightmap is {}.",
        risk
    );
}
