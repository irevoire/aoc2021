use std::str::FromStr;

use aoc::{Coord, Error};

#[derive(Debug)]
pub struct Vent {
    pub range: aoc::Range<usize>,
}

impl Vent {
    pub fn vertical(&self) -> bool {
        self.range.start.x == self.range.end.x
    }

    pub fn horizontal(&self) -> bool {
        self.range.start.y == self.range.end.y
    }

    pub fn orthogonal(&self) -> bool {
        self.vertical() || self.horizontal()
    }
}

impl FromStr for Vent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<Coord<usize>> = s.split(" -> ").map(|s| s.parse().unwrap()).collect();
        let [start, end] = [coords.iter().min().unwrap(), coords.iter().max().unwrap()];
        Ok(Self {
            range: start.to(*end)?,
        })
    }
}
