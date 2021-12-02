use aoc::*;

fn main() {
    let input: Vec<usize> = parser::lines().collect();
    let res = input
        .windows(2)
        .filter(|s| matches!(s, [a, b] if a < b))
        .count();

    let (first, _) = (1, 2);
    let [first, _] = [1, 2];
    let [first, _] = [1, 2].as_slice();

    answer!(
        "{} measurements are larger than the previous measurement.",
        res
    );
}
