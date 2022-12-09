#![feature(exclusive_range_pattern)]
const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for (i64, i64) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub(crate) struct Move(Direction, i64);

impl Move {
    fn from_string(string: &'static str) -> Self {
        let (direction, distance) = string.split_at(2);
        let direction = match direction.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Non-direction character found"),
        };

        Self(
            direction,
            distance
                .parse()
                .unwrap_or_else(|_| panic!("Found non-numeric input {distance}")),
        )
    }
}

pub(crate) fn distance_between_points(point_a: &(i64, i64), point_b: &(i64, i64)) -> (i64, i64) {
    (point_a.0 - point_b.0, point_a.1 - point_b.1)
}

pub(crate) fn parse_input(input: &'static str) -> Vec<Move> {
    input.lines().map(Move::from_string).collect()
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(&input);
    part_2(&input);
}
