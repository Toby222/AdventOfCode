pub const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &'static str) -> Vec<[u32; 2]> {
    let input = input
        .lines()
        .map(|line| -> [u32; 2] {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            [nums[0], nums[1]]
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

fn part_1(input: &Vec<[u32; 2]>) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for pair in input {
        left.push(pair[0]);
        right.push(pair[1]);
    }
    left.sort_unstable();
    right.sort_unstable();
    let mut diff = 0;
    for i in 0..left.len() {
        diff += u32::abs_diff(left[i], right[i]);
    }
    diff
}

fn part_2(input: &Vec<[u32; 2]>) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for pair in input {
        left.push(pair[0]);
        right.push(pair[1]);
    }

    let mut result = 0;

    for num in left {
        result += num * right.iter().filter(|&&x| x == num).count() as u32;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const SAMPLE_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part_1_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT);
        assert_eq!(super::part_1(&input), 11)
    }

    #[test]
    fn test_part_1_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_1(&input), 2192892);
    }

    #[test]
    fn test_part_2_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT);
        assert_eq!(super::part_2(&input), 31)
    }

    #[test]
    fn test_part_2_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_2(&input), 22962826);
    }
}
