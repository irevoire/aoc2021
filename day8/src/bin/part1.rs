fn main() {
    let res = aoc::parser::lines::<String>()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|s| s.trim().chars().count())
                .filter(|chars| matches!(chars, 2 | 3 | 4 | 7))
                .count()
        })
        .sum::<usize>();

    aoc::answer!("The digits 1, 4, 7, or 8 appear {} times.", res);
}
