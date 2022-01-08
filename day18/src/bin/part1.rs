#![feature(box_patterns)]

use std::fmt::Display;

use aoc::*;

fn main() {
    let snail: Snail = parser::lines::<String>()
        .map(|line| Snail::parse(&line.chars().collect::<Vec<char>>()).0)
        .reduce(|number, next| {
            let mut old_number = number.clone();
            let mut number = number.add(next);

            while old_number != number {
                old_number = number.clone();
                number = number.explode();
                if old_number != number {
                    continue;
                }
                number = number.split(&mut false);
            }

            number
        })
        .unwrap();

    answer!("The magnitude of the final sum is {}.", snail.magnitude());
}

#[derive(Debug, PartialEq, Clone)]
enum Snail {
    Pair { x: Box<Snail>, y: Box<Snail> },
    Number(usize),
}

use Snail::*;

impl Snail {
    pub fn parse(s: &[char]) -> (Self, &[char]) {
        if s[0] != '[' {
            return (Number(s[0].to_string().parse().unwrap()), &s[1..]);
        }

        let rem = &s[1..]; // skip the opening square bracket
        let (x, rem) = Self::parse(rem);
        let rem = &rem[1..]; // skip the comma
        let (y, rem) = Self::parse(rem);
        let rem = &rem[1..]; // skip the closing square bracket

        let (x, y) = (Box::new(x), Box::new(y));

        (Self::Pair { x, y }, rem)
    }

    fn get(&mut self, n: &mut usize) -> Option<&mut usize> {
        match self {
            Number(ref mut value) if *n == 0 => Some(value),
            Number(_) => {
                *n -= 1;
                None
            }
            Pair { x, y } => x.get(n).or_else(|| y.get(n)),
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Pair { x, y, .. } => 3 * x.magnitude() + 2 * y.magnitude(),
            Number(n) => *n,
        }
    }

    pub fn split(self, finished: &mut bool) -> Self {
        if *finished {
            self
        } else {
            match self {
                Number(n) if n < 10 => Number(n),
                Number(n) => {
                    *finished = true;
                    Pair {
                        x: Box::new(Number(n / 2)),
                        y: Box::new(Number(n / 2 + n % 2)),
                    }
                }
                Pair { x, y } => Pair {
                    x: Box::new(x.split(finished)),
                    y: Box::new(y.split(finished)),
                },
            }
        }
    }

    pub fn explode(mut self) -> Self {
        if let Some((position, left, right)) = self.inner_explode(&mut 0, 0) {
            if position > 0 {
                if let Some(left_node) = self.get(&mut (position.clone() - 1)) {
                    *left_node += left;
                }
            }
            if let Some(right_node) = self.get(&mut (position.clone() + 1)) {
                *right_node += right;
            }
        }

        self
    }

    // return (position, left, right)
    fn inner_explode(
        &mut self,
        position: &mut usize,
        depth: usize,
    ) -> Option<(usize, usize, usize)> {
        if depth >= 4 && !matches!(self, Number(_)) {
            match std::mem::replace(self, Number(0)) {
                Pair {
                    x: box Number(x),
                    y: box Number(y),
                } => Some((*position, x, y)),
                _ => None, // please
            }
        } else if matches!(self, Number(_)) {
            *position += 1;
            None
        } else if let Pair { x, y } = self {
            x.inner_explode(position, depth + 1)
                .or_else(|| y.inner_explode(position, depth + 1))
        } else {
            None
        }
    }

    pub fn add(self, rhs: Self) -> Self {
        Pair {
            x: Box::new(self),
            y: Box::new(rhs),
        }
    }
}

impl Display for Snail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n),
            Pair { x, y, .. } => write!(f, "[{},{}]", x, y),
        }
    }
}
