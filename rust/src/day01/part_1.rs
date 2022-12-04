pub(crate) fn part_1(input: &str) -> u32 {
    let maximum = input
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

#[test]
fn test_with_solution() {
    assert_eq!(part_1(&crate::INPUT.replace("\r\n", "\n")), 69528);
}
