pub(crate) fn part_1(input: &'static str) {
    let maximum = input
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
        .max()
        .expect("No highest inventory found. Input unclean?");

    println!("Part 1: {maximum}")
}
