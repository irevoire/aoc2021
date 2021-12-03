use aoc::*;

fn main() {
    let reports: Vec<_> = parser::lines::<String>()
        .map(|report| report.chars().collect::<Vec<char>>())
        .collect();

    let mut oxygen: Vec<Vec<usize>> = reports
        .iter()
        .map(|value| value.iter().map(|c| (*c == '1') as usize).collect())
        .collect();
    let mut co2 = oxygen.clone();

    for idx in 0.. {
        let most_common_bit = extract_most_common_bit(&oxygen, idx);
        oxygen.retain(|value| value[idx] == most_common_bit);
        if oxygen.len() == 1 {
            break;
        }
    }
    for idx in 0.. {
        let most_common_bit = extract_most_common_bit(&co2, idx);
        co2.retain(|value| value[idx] != most_common_bit);
        if co2.len() == 1 {
            break;
        }
    }

    let [oxygen, co2] = [oxygen[0].clone(), co2[0].clone()].map(|bits| {
        bits.into_iter()
            .reduce(|acc, bit| (acc << 1) + bit)
            .unwrap()
    });

    answer!("oxygen: {}.", oxygen);
    answer!("co2: {}.", co2);
    answer!(
        "the life support rating of the submarine is {}.",
        oxygen * co2
    );
}

pub fn extract_most_common_bit(reports: &[Vec<usize>], idx: usize) -> usize {
    let mut ones = 0;
    for report in reports {
        ones += (report[idx] == 1) as usize;
    }

    (ones >= (reports.len() - ones)) as usize
}
