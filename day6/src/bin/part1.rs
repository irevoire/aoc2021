use aoc::*;

fn main() {
    let mut fish: [usize; 9] = parser::input::<String>()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .fold([0; 9], |mut sea, fish: usize| {
            sea[fish] += 1;
            sea
        });

    for _ in 0..80 {
        fish.rotate_left(1);
        fish[6] += fish[8]
    }
    let fishes = fish.iter().sum::<usize>();

    answer!(
        "After 80 days there would be a total of {} lanternfish.",
        fishes
    );
}
