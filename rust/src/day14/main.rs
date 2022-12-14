#![feature(let_chains)]
#![feature(map_try_insert)]
#![feature(generators, generator_trait)]
const INPUT: &str = include_str!("input.txt");

use std::collections::HashSet;

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Debug, Clone)]
pub(crate) struct Cave {
    pub(crate) field: HashSet<[usize; 2]>,
    lowest_stone: usize,
}

impl Cave {
    fn new(field: HashSet<[usize; 2]>) -> Cave {
        let lowest_stone = field
            .iter()
            .map(|position| position[1])
            .max()
            .unwrap_or_default();
        Cave {
            field,
            lowest_stone,
        }
    }

    /// Inserts a field of sand into this [`Cave`], then returns whether it landed.
    pub(crate) fn insert_sand_void<const X: usize>(&mut self) -> Option<[usize; 2]> {
        let mut current_coordinate = [X, 0];

        loop {
            if current_coordinate[1] >= self.lowest_stone {
                return None;
            }

            let coordinate_below_straight = [current_coordinate[0], current_coordinate[1] + 1];
            if !self.field.contains(&coordinate_below_straight) {
                current_coordinate = coordinate_below_straight;
                continue;
            }

            let coordinate_below_left = [current_coordinate[0] - 1, current_coordinate[1] + 1];
            if !self.field.contains(&coordinate_below_left) {
                current_coordinate = coordinate_below_left;
                continue;
            }

            let coordinate_below_right = [current_coordinate[0] + 1, current_coordinate[1] + 1];
            if !self.field.contains(&coordinate_below_right) {
                current_coordinate = coordinate_below_right;
                continue;
            }

            self.field.insert(current_coordinate);
            return Some(current_coordinate);
        }
    }

    /// Inserts a field of sand into this [`Cave`], then returns where it landed.
    pub(crate) fn insert_sand_floor<const X: usize>(&mut self) -> [usize; 2] {
        let lowest_position = self.lowest_stone + 2;

        let mut current_coordinate = [X, 0];

        loop {
            let y_below = current_coordinate[1] + 1;
            if y_below < lowest_position {
                let coordinate_below_straight = [current_coordinate[0], y_below];
                if !self.field.contains(&coordinate_below_straight) {
                    current_coordinate = coordinate_below_straight;
                    continue;
                }

                let coordinate_below_left = [current_coordinate[0] - 1, y_below];
                if !self.field.contains(&coordinate_below_left) {
                    current_coordinate = coordinate_below_left;
                    continue;
                }

                let coordinate_below_right = [current_coordinate[0] + 1, y_below];
                if !self.field.contains(&coordinate_below_right) {
                    current_coordinate = coordinate_below_right;
                    continue;
                }

                assert!(self.field.insert(current_coordinate));
                return current_coordinate;
            } else {
                assert!(self.field.insert(current_coordinate));
                return current_coordinate;
            }
        }
    }
}

pub(crate) type Input = Cave;

pub(crate) fn interpolate(from: [usize; 2], to: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    let min_x = from[0].min(to[0]);
    let max_x = from[0].max(to[0]);
    let min_y = from[1].min(to[1]);
    let max_y = from[1].max(to[1]);
    (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| [x, y]))
}

fn parse_input(input: &'static str) -> Input {
    let result = input.lines().map(|line| {
        line.split(" -> ").map(|position_string| {
            let coordinates = position_string.split(',').collect::<Vec<_>>();
            assert_eq!(coordinates.len(), 2);

            [
                coordinates[0].parse::<usize>().unwrap(),
                coordinates[1].parse::<usize>().unwrap(),
            ]
        })
    });

    let mut stone_tiles = HashSet::new();

    for lines in result {
        let mut coordinates = lines;
        let mut current_point = coordinates.by_ref().next().unwrap();

        for destination in coordinates {
            for coordinate in interpolate(current_point, destination) {
                stone_tiles.insert(coordinate);
            }

            current_point = destination;
        }
    }

    Cave::new(stone_tiles)
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(input.clone());
    part_2(input);
}
