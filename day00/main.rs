pub const INPUT: &str = include_str!("./input.txt");

type Input = &'static str;
type Output = usize;

fn parse_input(input: &'static str) -> Input {
    input
}

pub fn main() {
    let input = parse_input(INPUT);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> Output {
    Output::default()
}

fn part_2(input: &Input) -> Output {
    Output::default()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Output};

    const SAMPLE_INPUT: &str = r#""#;

    #[test]
    fn test_part_1_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT);
        assert_eq!(super::part_1(&input), Output::default());
    }

    #[test]
    fn test_part_1_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_1(&input), Output::default());
    }

    #[test]
    fn test_part_2_with_sample_solution() {
        let input = parse_input(SAMPLE_INPUT);
        assert_eq!(super::part_2(&input), Output::default());
    }

    #[test]
    fn test_part_2_with_solution() {
        let input = parse_input(super::INPUT);
        assert_eq!(super::part_2(&input), Output::default());
    }
}
