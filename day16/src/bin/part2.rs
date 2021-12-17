use aoc::*;
use day16::*;

fn main() {
    let input = parser::input::<String>()
        .trim()
        .chars()
        .filter_map(|c| usize::from_str_radix(&c.to_string(), 16).ok())
        .flat_map(|c| {
            format!("{:04b}", c)
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();

    let packet = parse_packet(&input).0;

    answer!(
        "If we evaluate the expression represented by the transmission we get {}.",
        packet.value()
    );
}
