pub(crate) fn part_1(input: &str) -> u32 {
    let maximum = input
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
        .max()
        .expect("No highest inventory found. Input unclean?");

    println!("Part 1: {maximum}");
    maximum
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::INPUT.replace("\r\n", "\n")), 69528);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&SAMPLE_INPUT.replace("\r\n", "\n")), 24000);
    }
}
