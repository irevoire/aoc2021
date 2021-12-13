use aoc::*;

fn main() {
    let input = parser::input::<String>();
    let (dots, folds) = input.split_once("\n\n").unwrap();

    let mut dots: Vec<Coord<usize>> = dots.lines().map(|coord| coord.parse().unwrap()).collect();

    for fold in folds.lines().take(1) {
        let (_, fold) = fold.rsplit_once(' ').unwrap();
        let (axis, value) = fold.split_once('=').unwrap();
        let value: usize = value.parse().unwrap();

        match axis {
            "x" => dots
                .iter_mut()
                .filter(|coord| coord.x > value)
                .for_each(|coord| {
                    if coord.x % value == 0 {
                        coord.x = 0;
                    } else {
                        coord.x = value - (coord.x % value)
                    }
                }),
            "y" => dots
                .iter_mut()
                .filter(|coord| coord.y > value)
                .for_each(|coord| {
                    if coord.y % value == 0 {
                        coord.y = 0;
                    } else {
                        coord.y = value - (coord.y % value)
                    }
                }),
            _ => unreachable!(),
        }
    }

    dots.sort();
    dots.dedup();

    answer!(
        "{} dots are visible after completing just the first fold instruction.",
        dots.len()
    );
}
