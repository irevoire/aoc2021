use std::{ops::Add, str::FromStr};

use aoc::*;

pub enum Movement {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let movement = s.split_whitespace().collect::<Vec<_>>();
        Ok(match movement[0] {
            "forward" => Self::Forward(movement[1].parse()?),
            "up" => Self::Up(movement[1].parse()?),
            "down" => Self::Down(movement[1].parse()?),
            s => unreachable!(s),
        })
    }
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
