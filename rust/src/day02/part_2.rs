use std::str::FromStr;

use crate::{GameResult, Shape};

pub(crate) fn part_2(input: &'static str) -> u64 {
    let input = parse_input(input);
    let final_score = input
        .iter()
        .fold(0u64, |acc, (enemy_play, desired_result)| {
            acc + u64::from(&enemy_play.get_response_for_result(desired_result))
                + u64::from(desired_result)
        });
    println!("Part 2: {final_score}");
    final_score
}

fn parse_input(input: &'static str) -> Vec<(Shape, GameResult)> {
    input
        .lines()
        .map(|round_string| {
            let round = round_string.split(' ').take(2).collect::<Vec<_>>();
            assert_eq!(round.len(), 2);
            assert_eq!(round[0].len(), 1);
            assert_eq!(round[1].len(), 1);
            let enemy_shape = Shape::from_str(round[0])
                .unwrap_or_else(|_| panic!("Found invalid play {}", round[0]));
            let desired_result = GameResult::from_str(round[1])
                .unwrap_or_else(|_| panic!("Found invalid desired result {}", round[1]));

            (enemy_shape, desired_result)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 13131);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 12);
    }
}
