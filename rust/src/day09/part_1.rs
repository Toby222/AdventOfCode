use std::collections::HashSet;

use crate::Move;

pub(crate) fn part_1(input: &Vec<Move>) -> usize {
    let mut head_position = (0i64, 0i64);
    let mut tail_position = (0i64, 0i64);
    let mut visited_tail_positions = HashSet::new();

    for head_movement in input.iter() {
        for _ in 0..head_movement.1 {
            match head_movement.0 {
                crate::Direction::Up => head_position.1 += 1,
                crate::Direction::Down => head_position.1 -= 1,
                crate::Direction::Right => head_position.0 += 1,
                crate::Direction::Left => head_position.0 -= 1,
            }

            let point_distance = crate::distance_between_points(&head_position, &tail_position);

            match point_distance {
                (2, 0) => tail_position.0 += 1,
                (-2, 0) => tail_position.0 -= 1,
                (0, 2) => tail_position.1 += 1,
                (0, -2) => tail_position.1 -= 1,
                (_, 2) => tail_position = (head_position.0, tail_position.1 + 1),
                (2, _) => tail_position = (tail_position.0 + 1, head_position.1),
                (_, -2) => tail_position = (head_position.0, tail_position.1 - 1),
                (-2, _) => tail_position = (tail_position.0 - 1, head_position.1),
                _ => {}
            };

            visited_tail_positions.insert(tail_position);
        }
    }

    println!("Part 1: {:?}", visited_tail_positions.len());

    visited_tail_positions.len()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::parse_input(crate::INPUT)), 0);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&crate::parse_input(SAMPLE_INPUT)), 13);
    }
}
