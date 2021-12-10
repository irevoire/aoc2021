use day10::*;

fn main() {
    let mut scores = aoc::parser::lines::<String>()
        .filter_map(|line| errors(&line).ok())
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    ')' => 1u128,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                })
                .reduce(|acc, score| acc * 5 + score)
                .unwrap()
        })
        .collect::<Vec<_>>();

    scores.sort();

    let score = scores[scores.len() / 2];

    aoc::answer!("The middle score for all uncomplete lines is {}.", score);
}
