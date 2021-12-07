use aoc::*;

fn main() {
    let mut input = parser::input::<String>()
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    input.sort();

    let [min, max] = [*input.iter().min().unwrap(), *input.iter().max().unwrap()];

    let best_cost = (min..max)
        .map(|position| {
            input
                .iter()
                .map(|&crab| (1..=(position.max(crab) - position.min(crab))).sum::<usize>())
                .sum::<usize>()
        })
        .min()
        .unwrap();

    answer!(
        "It'll cost them {} fuel to align to that position.",
        best_cost
    );
}
