use aoc::*;

pub fn display_dots(dots: &[Coord<usize>]) {
    let width = dots.iter().map(|coord| coord.0).max().unwrap() + 1;
    let height = dots.iter().map(|coord| coord.1).max().unwrap() + 1;

    let mut grid: Grid<char> = Grid::with_dimension(width, height);
    grid.iter_mut().for_each(|p| *p = '.');

    dots.iter()
        .for_each(|point| *grid.get_mut(*point).unwrap() = '#');

    println!("{}", grid);
}
