use crate::CubeCounts;

impl CubeCounts {
    const fn exceeds_limits(&self) -> bool {
        self.red > 12 || self.green > 13 || self.blue > 14
    }
}

pub(crate) fn part_1(input: &'static str) -> usize {
    let id_sum = input
        .lines()
        .enumerate()
        .filter_map(|(game_id, line)| {
            let game_start_index = line.find(":").expect("Every line should have a ': '") + 2;

            let game = &line[game_start_index..];

            let groups = game.split("; ");

            for group in groups {
                let color_counts = group.split(", ");

                let group_counts = color_counts.map(|color_count| {
                    let (count, color) = color_count
                        .split_once(' ')
                        .expect("Every color count should have one space");

                    let count = count
                        .parse()
                        .expect("Every color count should be a valid number");

                    match color {
                        "red" => CubeCounts::red(count),
                        "green" => CubeCounts::green(count),
                        "blue" => CubeCounts::blue(count),
                        _ => unreachable!(),
                    }
                });
                if group_counts.sum::<CubeCounts>().exceeds_limits() {
                    return None;
                }
            }
            Some(game_id + 1)
        })
        .sum();
    id_sum
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 8)
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 2449);
    }
}
