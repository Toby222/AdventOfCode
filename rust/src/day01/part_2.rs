pub(crate) fn part_2(input: &str) -> u32 {
    let mut calories = input
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

#[test]
fn test_with_solution() {
    assert_eq!(part_2(&crate::INPUT.replace("\r\n", "\n")), 206152);
}
