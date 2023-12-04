use std::collections::HashSet;

struct Card {
    numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl Card {
    fn score(&self) -> usize {
        let winning_number_count =
            HashSet::intersection(&self.numbers, &self.winning_numbers).count();

        match winning_number_count {
            0 => 0,
            x => usize::pow(2, x as u32 - 1),
        }
    }
}

pub(crate) fn part_1(input: &'static str) -> usize {
    let prefix_end = input.find(':').expect("Input should start with 'Card  X:'") + 1;
    input
        .lines()
        .map(|line| {
            let mut number_groups = line[prefix_end..].split('|').map(|group| {
                group
                    .split(' ')
                    .filter_map(|number| {
                        if number.is_empty() {
                            None
                        } else {
                            Some(
                                number
                                    .trim()
                                    .parse::<u8>()
                                    .expect("Every number should be a valid u8"),
                            )
                        }
                    })
                    .collect()
            });
            let numbers = number_groups
                .next()
                .expect("There should be two groups of numbers");

            let winning_numbers = number_groups
                .next()
                .expect("There should be two groups of numbers");

            assert!(
                number_groups.next().is_none(),
                "There should be no more groups of numbers"
            );

            Card {
                numbers,
                winning_numbers,
            }
            .score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 13)
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 24160);
    }
}
