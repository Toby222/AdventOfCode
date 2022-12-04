#[inline(always)]
fn sections_contain_each_other(sections: [[u32; 2]; 2]) -> bool {
    let start_1 = sections[0][0];
    let end_1 = sections[0][1];
    let start_2 = sections[1][0];
    let end_2 = sections[1][1];

    (start_1 <= start_2 && end_1 >= end_2) || (start_1 >= start_2 && end_1 <= end_2)
}

pub(crate) fn part_1(input: &[[[u32; 2]; 2]]) -> usize {
    let times_contained = input
        .iter()
        .filter(|sections| sections_contain_each_other(**sections))
        .collect::<Vec<_>>()
        .len();
    println!("Part 1: {times_contained}");
    times_contained
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::parse_input(crate::INPUT)), 584);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&crate::parse_input(SAMPLE_INPUT)), 2);
    }

    #[test]
    #[ignore = "Sanity check test"]
    fn test_containment() {
        for num in 0..=1024u32 {
            assert!(super::sections_contain_each_other([[num, num], [num, num]]));
        }

        for num_a in 0..=1024u32 {
            for num_b in num_a..=1024u32 {
                assert!(super::sections_contain_each_other([
                    [num_a, num_b],
                    [num_b, num_b]
                ]));
                assert!(super::sections_contain_each_other([
                    [num_a, num_b],
                    [num_a, num_b]
                ]));
            }
        }

        for num_a in 0..=1024u32 {
            for num_b in num_a + 1u32..=num_a + 1024u32 {
                assert!(!super::sections_contain_each_other([
                    [num_a, num_a],
                    [num_b, num_b]
                ]));
            }
        }
        assert!(!super::sections_contain_each_other([[1, 3], [3, 83]]));
    }
}
