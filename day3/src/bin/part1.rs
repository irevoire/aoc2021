use aoc::*;

fn main() {
    let reports: Vec<_> = parser::lines::<String>()
        .map(|report| report.chars().collect::<Vec<char>>())
        .collect();
    let gamma = extract_most_common_bit(&reports);
    let epsilon: Vec<usize> = gamma.iter().map(|bit| 1 - *bit).collect();

    let [gamma, epsilon] = [gamma, epsilon].map(|bits| {
        bits.into_iter()
            .reduce(|acc, bit| (acc << 1) + bit)
            .unwrap()
    });
    answer!("gamma: {}.", gamma);
    answer!("epsilon: {}.", epsilon);
    answer!("The power consumption is {}.", gamma * epsilon);
}

fn extract_most_common_bit(reports: &[Vec<char>]) -> Vec<usize> {
    let mut gamma = vec![0; reports[0].len()];
    for report in reports {
        for i in 0..report.len() {
            gamma[i] += (report[i] == '1') as usize;
        }
    }

    gamma
        .iter()
        .map(|ones| (*ones > (reports.len() / 2)) as usize)
        .collect()
}
