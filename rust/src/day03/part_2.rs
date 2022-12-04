use std::collections::HashSet;

use crate::character_priority;

pub(crate) fn part_2(input: &'static str) -> u64 {
    let rucksacks = input.lines();
    let compartments = rucksacks
        .map(|rucksack| rucksack.bytes().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    assert_eq!(compartments.len() % 3, 0);

    let mut running_sum = 0;
    for group_compartments in compartments.chunks_exact(3) {
        let group_bytes = &group_compartments[0]
            .intersection(&group_compartments[1])
            .copied()
            .filter(|byte| group_compartments[2].contains(byte))
            .collect::<Vec<_>>();
        assert_eq!(group_bytes.len(), 1);
        running_sum += character_priority(group_bytes[0]);
    }
    println!("Part 2: {running_sum}");
    running_sum
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 2276);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 70);
    }
}
