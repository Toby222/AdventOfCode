#[derive(Default, Debug)]
struct CubeCounts {
    red: u16,
    green: u16,
    blue: u16,
}
impl std::fmt::Display for CubeCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.red, self.green, self.blue)
    }
}

impl std::ops::Add<CubeCounts> for CubeCounts {
    type Output = Self;

    fn add(self, rhs: CubeCounts) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::iter::Sum<CubeCounts> for CubeCounts {
    fn sum<I: Iterator<Item = CubeCounts>>(iter: I) -> Self {
        iter.fold(CubeCounts::default(), |acc, x| acc + x)
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

            let game_counts = groups
                .map(|group| {
                    let color_counts = group.split(", ");

                    let group_counts =
                        color_counts.map(|color_count| {
                            let (count, color) = color_count
                                .split_once(' ')
                                .expect("Every color count should have one space");

                            let count = count.parse().expect(
                        format!("Every color count should be a valid number - `{count}` `{color}`")
                            .as_str(),
                    );

                            match color {
                                "red" => CubeCounts {
                                    red: count,
                                    ..Default::default()
                                },
                                "green" => CubeCounts {
                                    green: count,
                                    ..Default::default()
                                },
                                "blue" => CubeCounts {
                                    blue: count,
                                    ..Default::default()
                                },
                                _ => unreachable!(),
                            }
                        });
                    group_counts.sum::<CubeCounts>()
                })
                .reduce(|acc, cube_count| CubeCounts {
                    red: u16::max(acc.red, cube_count.red),
                    green: u16::max(acc.green, cube_count.green),
                    blue: u16::max(acc.blue, cube_count.blue),
                })
                .expect("At least one group should exist in each game");
            if game_counts.red <= 12 && game_counts.green <= 13 && game_counts.blue <= 14 {
                Some(game_id + 1)
            } else {
                None
            }
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
