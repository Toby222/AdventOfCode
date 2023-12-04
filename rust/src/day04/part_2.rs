use std::collections::HashSet;

struct Card {
    times_won: usize,
    numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl Card {
    fn winning_number_count(&self) -> usize {
        HashSet::intersection(&self.numbers, &self.winning_numbers).count()
    }
}

pub(crate) fn part_2(input: &'static str) -> usize {
    let prefix_end = input.find(':').expect("Input should start with 'Card  X:'") + 1;
    let mut cards = input
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
                times_won: 1,
                numbers,
                winning_numbers,
            }
        })
        .collect::<Vec<_>>();

    for idx in 0..cards.len() {
        let card_score = cards[idx].winning_number_count();

        for winning_idx in (idx + 1)..(idx + 1 + card_score) {
            cards[winning_idx].times_won += cards[idx].times_won;
        }
    }

    cards.iter().map(|card| card.times_won).sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 30);
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 5659035);
    }
}
