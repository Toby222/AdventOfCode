use std::collections::HashSet;

pub(crate) fn part_1(input: &'static str) -> u64 {
    let rucksacks = input.lines();
    let compartments = rucksacks.map(|rucksack| {
        let compartment_size = rucksack.len() / 2;
        let compartments = (
            rucksack
                .bytes()
                .take(compartment_size)
                .collect::<HashSet<_>>(),
            rucksack
                .bytes()
                .skip(compartment_size)
                .take(compartment_size)
                .collect::<HashSet<_>>(),
        );
        compartments
    });
    let mut running_sum = 0;
    for compartments in compartments {
        let same_bytes =
            HashSet::intersection(&compartments.0, &compartments.1).collect::<Vec<_>>();
        assert_eq!(
            same_bytes.len(),
            1,
            "{}",
            same_bytes
                .iter()
                .map(|byte| char::from(**byte))
                .collect::<String>()
        );
        running_sum += crate::character_priority(*same_bytes[0]);
    }
    println!("Part 1: {running_sum}");
    running_sum
}

#[test]
fn test_with_solution() {
    assert_eq!(part_1(crate::INPUT), 7742);
}
