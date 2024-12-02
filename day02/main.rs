#![feature(array_windows)]
pub const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &'static str) -> Vec<Vec<i32>> {
    let input = input
        .lines()
        .map(|line| -> Vec<i32> {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    input
}

pub fn main() {
    let input = parse_input(INPUT);
    let result_1 = part_1(&input);
    println!("{result_1}");
    let result_2 = part_2(&input);
    println!("{result_2}");
}

fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report[0] < report[1];
    report.array_windows::<2>().all(|window| {
        let increase = window[1] - window[0];
        if increasing {
            1 <= increase && increase <= 3
        } else {
            -1 >= increase && increase >= -3
        }
    })
}

fn part_1(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|&report| is_safe(report)).count()
}

fn part_2(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .filter(|report| {
            (0..report.len())
                .map(|i| [&report[..i], &report[i + 1..]].concat())
                .any(|modified_report| is_safe(&modified_report))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const SAMPLE_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part_1_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT);
        assert_eq!(super::part_1(&input), 2)
    }

    #[test]
    fn test_part_1_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_1(&input), 379);
    }

    #[test]
    fn test_part_2_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT);
        assert_eq!(super::part_2(&input), 4)
    }

    #[test]
    fn test_part_2_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_2(&input), 430);
    }
}
