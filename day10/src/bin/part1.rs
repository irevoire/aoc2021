use day10::*;

fn main() {
    let score = aoc::parser::lines::<String>()
        .map(|line| errors(&line))
        .filter(|error| error.is_err())
        .map(|error| error.unwrap_err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum::<usize>();

    aoc::answer!(
        "The total syntax error score for those errors is {}.",
        score
    );
}
