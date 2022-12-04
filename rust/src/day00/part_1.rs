pub(crate) fn part_1(_input: &'static str) -> u64 {
    todo!("Part 1")
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(
            super::part_1(crate::INPUT),
            todo!("Add result for solved part 1")
        );
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(
            super::part_1(SAMPLE_INPUT),
            todo!("Add result from example part 1")
        );
    }
}
