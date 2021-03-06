use std::ops::Add;

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
#[derive(Default)]
pub struct Position {
    pub forward: isize,
    pub depth: isize,
}

impl Add<Movement> for Position {
    type Output = Self;

    fn add(mut self, rhs: Movement) -> Self::Output {
        match rhs {
            Movement::Forward(n) => self.forward += n,
            Movement::Up(n) => self.depth -= n,
            Movement::Down(n) => self.depth += n,
        }
        self
    }
}
