use std::cmp::Reverse;

const DIGIT_PATTERNS: [(char, &str); 10] = [
    ('0', "zero"),
    ('1', "one"),
    ('2', "two"),
    ('3', "three"),
    ('4', "four"),
    ('5', "five"),
    ('6', "six"),
    ('7', "seven"),
    ('8', "eight"),
    ('9', "nine"),
];

#[cfg_attr(test, derive(Debug))]
struct DigitIndices {
    digit: u8,
    first_index: usize,
    last_index: usize,
}

pub(crate) fn part_2(input: &'static str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digit_indices = DIGIT_PATTERNS
                .iter()
                .filter_map(|(digit, pattern)| {
                    let first_index_digit = line.find(*digit).map(Reverse);
                    let first_index_str = line.find(pattern).map(Reverse);
                    let Reverse(first_index) = Option::max(first_index_digit, first_index_str)?;

                    let last_index_digit = line.rfind(*digit);
                    let last_index_str = line.rfind(pattern);
                    let last_index = Option::max(last_index_digit, last_index_str)?;

                    Some(DigitIndices {
                        digit: unsafe { digit.to_digit(10).unwrap_unchecked() } as u8,
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

            (first_digit as u32 * 10) + last_digit as u32
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
