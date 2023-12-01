pub(crate) fn part_1(input: &'static str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(char::is_ascii_digit);
            let first_digit = digits.next().expect("At least 1 digit should be present");
            let last_digit = digits.last().unwrap_or(first_digit);

            format!("{first_digit}{last_digit}")
                .parse::<u64>()
                .expect("Two digits should make a number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input_part_1.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 142)
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 55488);
    }
}
