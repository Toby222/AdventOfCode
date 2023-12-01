use std::cmp::Reverse;

const DIGIT_PATTERNS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg_attr(test, derive(Debug))]
struct DigitIndices {
    digit: usize,
    first_index: usize,
    last_index: usize,
}

pub(crate) fn part_2(input: &'static str) -> usize {
    input
        .lines()
        .map(|line| {
            let digit_indices = DIGIT_PATTERNS
                .iter()
                .enumerate()
                .filter_map(|(digit, pattern)| {
                    // SAFETY: Generating a char from a number like this is guaranteed to be in ASCII range
                    let first_index_digit = line
                        .find(unsafe {
                            char::from_u32(digit as u32 + b'0' as u32).unwrap_unchecked()
                        })
                        .map(Reverse);
                    let first_index_str = line.find(pattern).map(Reverse);
                    let Reverse(first_index) = Option::max(first_index_digit, first_index_str)?;

                    // SAFETY: Generating a char from a number like this is guaranteed to be in ASCII range
                    let last_index_digit = line.rfind([unsafe {
                        char::from_u32(digit as u32 + b'0' as u32).unwrap_unchecked()
                    }]);
                    let last_index_str = line.rfind(pattern);
                    let last_index = Option::max(last_index_digit, last_index_str)?;

                    Some(DigitIndices {
                        digit,
                        first_index,
                        last_index,
                    })
                })
                .collect::<Vec<_>>();
            // SAFETY: Each line is guaranteed to have at least one digit
            let first_digit = unsafe {
                digit_indices
                    .iter()
                    .min_by_key(|val| val.first_index)
                    .unwrap_unchecked()
            }
            .digit;
            let last_digit = unsafe {
                digit_indices
                    .iter()
                    .max_by_key(|val| val.last_index)
                    .unwrap_unchecked()
            }
            .digit;

            (first_digit * 10) + last_digit
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
