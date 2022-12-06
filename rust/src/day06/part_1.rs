const STRING_SLICE_LENGTH: usize = 4;

pub(crate) fn part_1(input: &'static str) -> usize {
    for idx in 0..(input.len() - STRING_SLICE_LENGTH) {
        let mut unique_chars = input[idx..idx + STRING_SLICE_LENGTH]
            .chars()
            .collect::<Vec<_>>();
        unique_chars.sort();
        unique_chars.dedup();
        if unique_chars.len() == STRING_SLICE_LENGTH {
            return idx + STRING_SLICE_LENGTH;
        }
    }
    unreachable!("We should always find an answer");
}

#[cfg(test)]
mod tests {
    const SAMPLES: [(&str, usize); 5] = [
        (include_str!("sample_inputs/1.txt"), 7),
        (include_str!("sample_inputs/2.txt"), 5),
        (include_str!("sample_inputs/3.txt"), 6),
        (include_str!("sample_inputs/4.txt"), 10),
        (include_str!("sample_inputs/5.txt"), 11),
    ];

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 1723);
    }

    #[test]
    fn test_with_sample_solution() {
        for (sample_input, sample_answer) in SAMPLES {
            assert_eq!(super::part_1(sample_input), sample_answer);
        }
    }
}
