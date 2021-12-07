use aoc::*;

fn main() {
    let mut input = parser::input::<String>()
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    input.sort();

    let best_position = input[input.len() / 2];
    let total_cost: usize = input
        .iter()
        .map(|&position| position.max(best_position) - position.min(best_position))
        .sum();

    answer!("The best position is {}.", best_position);
    println!(
        "It'll cost them {} fuel to align to that position.",
        total_cost
    );
}
