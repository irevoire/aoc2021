use aoc::*;

fn main() {
    let input = parser::input::<String>();
    let input = input.trim_start_matches("target area: x=");
    let (x, y) = input.split_once(", ").unwrap();
    let y = y.trim_start_matches("y=");

    let [x, y] = [x, y]
        .map(|coord| coord.split_once("..").unwrap())
        .map(|(start, end)| {
            (
                start.parse::<isize>().unwrap(),
                end.trim().parse::<isize>().unwrap(),
            )
        });
    let start = Coord::at(x.0, y.0);
    let end = Coord::at(x.1, y.1);

    let target = start.to(end).unwrap();

    let res = (0..1000)
        .zip(0..1000)
        .filter_map(|coord| Probe::new(coord).hit(target.clone()))
        .max()
        .unwrap();

    answer!(
        "The highest y position the probe reaches on its trajectory is {}.",
        res
    );
}

#[derive(Clone)]
struct Probe {
    pos: Coord<isize>,
    velocity: Coord<isize>,
}

impl Probe {
    pub fn new(velocity: impl Into<Coord<isize>>) -> Self {
        Self {
            pos: Coord::default(),
            velocity: velocity.into(),
        }
    }

    pub fn hit(mut self, target: Range<isize>) -> Option<isize> {
        let mut max_y = self.pos.y;

        while self.pos.y >= target.start.y {
            max_y = max_y.max(self.pos.y);

            if target.contains(self.pos) || target.start.y == self.pos.y {
                return Some(max_y);
            }
            self.pos.x += self.velocity.x;
            self.pos.y += self.velocity.y;
            self.velocity.x -= self.velocity.x.signum();
            self.velocity.y -= 1;
        }

        None
    }
}
