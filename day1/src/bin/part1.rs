use aoc::*;

fn main() {
    let input: Vec<usize> = parser::lines().collect();
    let res = input.windows(2).filter(|w| w[0] < w[1]).count();

    answer!(
        "{} measurements are larger than the previous measurement.",
        res
    );
}
