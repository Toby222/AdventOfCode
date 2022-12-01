pub(crate) fn part_2(input: &'static str) {
    let mut calories = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split('\n')
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
}
