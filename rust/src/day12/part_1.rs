use crate::{dijkstra, ParsedInput};

pub(crate) fn part_1(input: &ParsedInput) -> usize {
    let ParsedInput {
        start_position,
        end_position,
        grid,
    } = input;

    let path_length = dijkstra(grid, *start_position, *end_position).expect("All inputs should be solvable").len();
    // The first step doesn't count, ig?
    println!("Part 1: {}", path_length);

    path_length
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::parse_input(crate::INPUT)), 462);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&crate::parse_input(SAMPLE_INPUT)), 31);
    }
}
