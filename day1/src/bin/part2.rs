use aoc::*;

fn main() {
    let input: Vec<usize> = parser::lines().collect();
    let windows: Vec<usize> = input.windows(3).map(|w| w.iter().sum()).collect();
    let res = windows.windows(2).filter(|w| w[0] < w[1]).count();

    answer!("There is {} sums larger than the previous sum.", res);
}
