pub(crate) fn part_2(input: &'static str) -> usize {
    let trees = input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<_>>())
        .enumerate()
        .collect::<Vec<_>>();
    let max_y = trees.len();
    let max = trees
        .iter()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(|(x, v)| {
                let y = *y;
                if y == 0 || x == 0 || x >= line.len() - 1 || y >= max_y - 1 {
                    return 0;
                }

                let left = &line[..x];
                let right = &line[(x + 1)..];
                let up = (0..y).map(|i| trees[i].1[x]);
                let mut down = ((y + 1)..max_y).map(|i| trees[i].1[x]);

                let score = |pos: Option<usize>, default: usize| pos.unwrap_or(default) + 1;

                score(up.rev().position(|n| n >= *v), y.saturating_sub(1))
                    * score(down.position(|n| n >= *v), max_y.saturating_sub(y + 2))
                    * score(left.iter().rev().position(|n| n >= v), x.saturating_sub(1))
                    * score(
                        right.iter().position(|n| n >= v),
                        line.len().saturating_sub(x + 2),
                    )
            })
        })
        .max()
        .unwrap();

    println!("Part 2: {max}");

    max
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 0);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 8);
    }
}
