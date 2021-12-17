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

    let possibilities = (0..1000)
        .cartesian_product(-1000..1000)
        .filter(|coord| Probe::new(*coord).hit(target.clone()).is_some())
        .count();

    answer!(
        "There is {} initial distinct velocity values that cause the probe to hit the target.",
        possibilities
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

            if target.contains(self.pos) {
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
