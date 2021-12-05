use aoc::*;

use std::result::Result as StdResult;
use std::str::FromStr;

type Board = Grid<Result<usize, usize>>;

fn parse_grid(s: &str) -> Board {
    let inner = s
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|cell| Err(cell.parse().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Grid::from(inner)
}

pub fn is_win(grid: &Board) -> bool {
    grid.lines()
        .any(|line| line.iter().all(|cell| cell.is_ok()))
        || grid
            .columns()
            .any(|line| line.iter().all(|cell| cell.is_ok()))
}

pub fn score(grid: &Board) -> usize {
    grid.iter()
        .filter(|cell| cell.is_err())
        .map(|cell| cell.unwrap_err())
        .sum()
}

pub fn play(grid: &mut Board, n: usize) {
    grid.iter_mut()
        .filter(|cell| matches!(cell, Err(c) if *c == n))
        .for_each(|cell| *cell = StdResult::Ok(n))
}

#[derive(Debug)]
pub struct Bingo {
    current_number: usize,
    numbers: Vec<usize>,
    pub grids: Vec<Board>,
}

impl FromStr for Bingo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split("\n\n");
        let numbers = chunks
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let grids = chunks.map(|chunk| parse_grid(chunk)).collect();

        Ok(Self {
            current_number: 0,
            numbers,
            grids,
        })
    }
}

impl Bingo {
    pub fn play_turn(&mut self) {
        let current_number = self.numbers[self.current_number];
        self.current_number += 1;
        self.grids
            .iter_mut()
            .for_each(|grid| play(grid, current_number))
    }

    pub fn win(&self) -> Option<&Board> {
        self.grids.iter().find(|grid| is_win(grid))
    }

    pub fn current_number(&self) -> usize {
        // we are always one number late
        self.numbers[self.current_number - 1]
    }
}
