use std::collections::HashSet;

fn main() {
    let input: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> = aoc::parser::lines::<String>()
        .map(|line| {
            let mut line = line.split('|').map(|parts| {
                parts
                    .split(' ')
                    .map(|s| s.trim().chars().collect::<HashSet<char>>())
                    .collect::<Vec<_>>()
            });
            (line.next().unwrap(), line.next().unwrap())
        })
        .collect();

    let mut sum = 0;

    for line in input {
        let mut mixed: Vec<HashSet<char>> = line.0.iter().chain(&line.1).cloned().collect();
        mixed.sort_by_key(|word| word.len());
        // I don’t know why I have two empty cells
        mixed.drain(0..2);

        let one = mixed.iter().find(|word| word.len() == 2).unwrap();
        let seven = mixed.iter().find(|word| word.len() == 3).unwrap();
        let four = mixed.iter().find(|word| word.len() == 4).unwrap();
        let eight = mixed.iter().find(|word| word.len() == 7).unwrap();

        // we are looking for the six, it has two segments in common with the 7 but one is missing.
        // Only the 9 and 0 has the same number of segment, but they all are superset of 7.
        let six = mixed
            .iter()
            .filter(|word| word.len() == 6)
            .find(|six| !six.is_superset(&seven))
            .unwrap();

        // we are looking for the zero, it has three segments in common with the four but one is missing.
        let zero = mixed
            .iter()
            .filter(|&word| word.len() == 6 && word != six)
            .find(|zero| !zero.is_superset(four))
            .unwrap();

        let nine = mixed
            .iter()
            .filter(|&word| word.len() == 6 && word != six && word != zero)
            .next()
            .unwrap();

        let top = *seven.difference(one).next().unwrap();
        let top_right = *seven.difference(six).next().unwrap();
        let bottom_right = *one.intersection(six).next().unwrap();
        let middle = *four.difference(zero).next().unwrap();
        let top_left = *four
            .iter()
            .filter(|segment| ![middle, top_right, bottom_right].contains(segment))
            .next()
            .unwrap();
        let bottom = *nine
            .iter()
            .filter(|segment| ![top, middle, top_right, bottom_right, top_left].contains(segment))
            .next()
            .unwrap();
        let bottom_left = *eight
            .iter()
            .filter(|segment| {
                ![top, middle, top_right, bottom_right, top_left, bottom].contains(segment)
            })
            .next()
            .unwrap();

        let two: HashSet<char> = [top, top_right, middle, bottom_left, bottom]
            .into_iter()
            .collect();

        let three: HashSet<char> = [top, top_right, middle, bottom_right, bottom]
            .into_iter()
            .collect();

        let five: HashSet<char> = [top, top_left, middle, bottom_right, bottom]
            .into_iter()
            .collect();

        // println!("{:?}: 0", zero);
        // println!("{:?}: 1", one);
        // println!("{:?}: 2", two);
        // println!("{:?}: 3", three);
        // println!("{:?}: 4", four);
        // println!("{:?}: 5", five);
        // println!("{:?}: 6", six);
        // println!("{:?}: 7", seven);
        // println!("{:?}: 8", eight);
        // println!("{:?}: 9", nine);

        let res = line
            .1
            .iter()
            .filter(|digit| !digit.is_empty())
            .map(|digit| match digit {
                digit if digit == zero => 0,
                digit if digit == one => 1,
                digit if *digit == two => 2,
                digit if *digit == three => 3,
                digit if digit == four => 4,
                digit if *digit == five => 5,
                digit if digit == six => 6,
                digit if digit == seven => 7,
                digit if digit == eight => 8,
                digit if digit == nine => 9,
                digit => unreachable!("{:?}", digit),
            })
            .fold(0, |acc, digit| acc * 10 + digit);

        sum += res;
    }
    aoc::answer!("When you add up all the output values we get {}.", sum);
}
