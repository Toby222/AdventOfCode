pub(crate) fn part_2(mut input: crate::Input) -> u64 {
    // Increase by 1 because I break as soon as (500, 0) is placed, so it doesn't get counted
    let mut sand_count = 1;
    while let inserted_at = input.insert_sand_floor::<500>() && inserted_at[1] != 0 {
        sand_count += 1;
    }

    println!("Part 2: {sand_count}");

    sand_count
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::parse_input(crate::INPUT)), 29044);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(crate::parse_input(SAMPLE_INPUT)), 93);
    }
}
