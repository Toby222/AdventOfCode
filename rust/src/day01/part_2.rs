const DIGIT_PATTERNS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub(crate) fn part_2(input: &'static str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut digit_indices = DIGIT_PATTERNS
                .iter()
                .enumerate()
                .map(|(number, pattern)| {
                    let first_index_digit = line.find(&number.to_string());
                    let first_index_str = line.find(pattern);
                    let first_index = match (first_index_str, first_index_digit) {
                        (Some(lhs), Some(rhs)) => Some(usize::min(lhs, rhs)),
                        (None, idx) => idx,
                        (idx, None) => idx,
                    };

                    let last_index_digit = line.rfind(&number.to_string());
                    let last_index_str = line.rfind(pattern);
                    let last_index = match (last_index_str, last_index_digit) {
                        (Some(lhs), Some(rhs)) => Some(usize::max(lhs, rhs)),
                        (None, idx) => idx,
                        (idx, None) => idx,
                    };

                    (number, first_index, last_index)
                })
                .collect::<Vec<_>>();
            digit_indices.sort_unstable_by(|a, b| match (a.1, b.1) {
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
                (Some(lhs), Some(rhs)) => lhs.cmp(&rhs),
            });
            // SAFETY: Slice is guaranteed to be length 10
            let first_digit = unsafe { digit_indices.first().unwrap_unchecked() }.0;
            digit_indices.sort_unstable_by(|a, b| match (a.2, b.2) {
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
                (Some(lhs), Some(rhs)) => rhs.cmp(&lhs),
            });
            let last_digit = unsafe { digit_indices.first().unwrap_unchecked() }.0;
            format!("{first_digit}{last_digit}")
                .parse::<u64>()
                .expect("digits are guaranteed to create a valid number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input_part_2.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 281);
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 55614);
    }
}
