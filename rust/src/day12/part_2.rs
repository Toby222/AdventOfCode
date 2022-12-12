use crate::{dijkstra, ParsedInput};

pub(crate) fn part_2(input: &ParsedInput) -> usize {
    let ParsedInput {
        start_position: _,
        end_position,
        grid,
    } = input;

    let mut idx = 0;
    let shortest_path_length = grid
        .iter()
        .filter_map(|(position, height)| {
            println!("{idx}");
            idx += 1;
            if *height != 0 {
                None
            } else {
                dijkstra(grid, *position, *end_position)
            }
        })
        .fold(usize::MAX, |accumulator, path| accumulator.min(path.len()));
    // The first step doesn't count, ig?
    println!("Part 2: {}", shortest_path_length);

    shortest_path_length
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(&crate::parse_input(crate::INPUT)), 451);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(&crate::parse_input(SAMPLE_INPUT)), 29);
    }
}
