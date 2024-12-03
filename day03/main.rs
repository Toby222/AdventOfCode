#![feature(array_windows)]

use regex_lite::Regex;

pub const INPUT: &str = include_str!("./input.txt");

type Input = &'static str;

fn parse_input(input: &'static str) -> Input {
    input
}

pub fn main() {
    let input = parse_input(INPUT);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    const PATTERN: &str = r"mul\((\d{1,3})\,(\d{1,3})\)";
    let regex = Regex::new(PATTERN).expect("I fucked up the regex");
    assert!(regex.is_match(INPUT));

    let mut sum = 0;
    for r#match in regex.captures_iter(input) {
        let first = r#match
            .get(1)
            .expect("Missing first number")
            .as_str()
            .parse::<usize>()
            .expect("Failed to parse first number:");
        let second = r#match
            .get(2)
            .expect("Missing second number")
            .as_str()
            .parse::<usize>()
            .expect("Failed to parse second number:");
        sum += first * second;
    }
    sum
}

fn part_2(input: &Input) -> usize {
    const PATTERN: &str = r"mul\((\d{1,3})\,(\d{1,3})\)|do\(\)|don't\(\)";
    let regex = Regex::new(PATTERN).expect("I fucked up the regex");
    assert!(regex.is_match(INPUT));

    let mut sum = 0;
    let mut enabled = true;
    for r#match in regex.captures_iter(input) {
        match r#match.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                let first = r#match
                    .get(1)
                    .expect("Missing first number")
                    .as_str()
                    .parse::<usize>()
                    .expect("Failed to parse first number:");
                let second = r#match
                    .get(2)
                    .expect("Missing second number")
                    .as_str()
                    .parse::<usize>()
                    .expect("Failed to parse second number:");
                sum += first * second;
            }
            _ => {}
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const SAMPLE_INPUT_1: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const SAMPLE_INPUT_2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_part_1_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT_1);
        assert_eq!(super::part_1(&input), 161)
    }

    #[test]
    fn test_part_1_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_1(&input), 173529487);
    }

    #[test]
    fn test_part_2_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT_2);
        assert_eq!(super::part_2(&input), 48)
    }

    #[test]
    fn test_part_2_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_2(&input), 99532691);
    }
}
