pub(crate) fn part_2(_input: &'static str) -> u64 {
    todo!("Part 2")
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(
            super::part_2(SAMPLE_INPUT),
            todo!("Add result from example part 2")
        );
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(
            super::part_2(crate::INPUT),
            todo!("Add result for solved part 2")
        );
    }
}
