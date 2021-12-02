use aoc::{answer, parser};
use day2::*;

fn main() {
    let end_position = parser::lines::<Movement>()
        .fold(Position::default(), |movement, position| {
            movement + position
        });

    answer!(
        "The multiplication of the horizontal position and depth is {}.",
        end_position.forward * end_position.depth
    );
}
