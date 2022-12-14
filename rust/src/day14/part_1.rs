pub(crate) fn part_1(mut input: crate::Input) -> u64 {
    let mut sand_count = 0;
    while let Some(_inserted_at) = input.insert_sand_void::<500>() {
        sand_count += 1;
    }

    println!("Part 1: {sand_count}");

    sand_count
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::parse_input(crate::INPUT)), 0);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(crate::parse_input(SAMPLE_INPUT)), 24);
    }
}
