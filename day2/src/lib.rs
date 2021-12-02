use std::str::FromStr;

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
