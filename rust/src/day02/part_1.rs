use std::str::FromStr;

use crate::Shape;

pub(crate) fn part_1(input: &'static str) -> u64 {
    let parsed_input = parse_input(input);
    let final_score = parsed_input.iter().fold(0u64, |acc, [enemy, mine]| {
        acc + u64::from(&mine.play_against(enemy)) + u64::from(mine)
    });
    println!("Part 1: {final_score}");
    final_score
}

pub(crate) fn parse_input(input: &'static str) -> Vec<[Shape; 2]> {
    input
        .lines()
        .map(|round_string| {
            let round = round_string
                .split(' ')
                .map(|play| {
                    assert_eq!(play.len(), 1, "play: {play}");
                    Shape::from_str(play).unwrap_or_else(|_| panic!("Found invalid play {play}"))
                })
                .take(2)
                .collect::<Vec<_>>();
            [round[0], round[1]]
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");
    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 13221);
    }
    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 15);
    }
}
