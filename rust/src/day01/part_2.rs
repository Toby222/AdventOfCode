pub(crate) fn part_2(input: &str) -> u32 {
    let mut calories = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .map(|item| {
                    item.parse::<u32>()
                        .expect("Input isn't clean. Non-number found")
                })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories.sort();
    calories.reverse();
    let top_three = calories.iter().take(3).sum::<u32>();

    println!("Part 2: {top_three}");
    top_three
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(&crate::INPUT.replace("\r\n", "\n")), 206152);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(&SAMPLE_INPUT.replace("\r\n", "\n")), 45000);
    }
}
