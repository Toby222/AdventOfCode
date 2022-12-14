#![feature(let_chains)]
#![feature(map_try_insert)]
#![feature(generators, generator_trait)]
const INPUT: &str = include_str!("input.txt");

use std::collections::HashMap;

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Material {
    Stone,
    Sand,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Cave {
    pub(crate) field: HashMap<(usize, usize), Material>,
}

impl Cave {
    fn lowest_stone(&self) -> usize {
        self.field
            .iter()
            .filter_map(|(position, material)| {
                if matches!(material, Material::Stone) {
                    Some(position.1)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_default()
    }

    /// Inserts a field of sand into this [`Cave`], then returns whether it landed.
    pub(crate) fn insert_sand_void<const X: usize>(&mut self) -> Option<(usize, usize)> {
        let lowest_position = self.lowest_stone();

        let mut current_coordinate = (X, 0);

        loop {
            if current_coordinate.1 >= lowest_position {
                return None;
            }

            let coordinate_below_straight = (current_coordinate.0, current_coordinate.1 + 1);
            let field_below_straight = self.field.get(&coordinate_below_straight);
            if field_below_straight.is_none() {
                current_coordinate = coordinate_below_straight;
                continue;
            }

            let coordinate_below_left = (current_coordinate.0 - 1, current_coordinate.1 + 1);
            let field_below_left = self.field.get(&coordinate_below_left);
            if field_below_left.is_none() {
                current_coordinate = coordinate_below_left;
                continue;
            }

            let coordinate_below_right = (current_coordinate.0 + 1, current_coordinate.1 + 1);
            let field_below_right = self.field.get(&coordinate_below_right);
            if field_below_right.is_none() {
                current_coordinate = coordinate_below_right;
                continue;
            }

            self.field.insert(current_coordinate, Material::Sand);
            return Some(current_coordinate);
        }
    }

    /// Inserts a field of sand into this [`Cave`], then returns where it landed.
    pub(crate) fn insert_sand_floor<const X: usize>(&mut self) -> (usize, usize) {
        let lowest_position = self.lowest_stone() + 2;

        let mut current_coordinate = (X, 0);

        loop {
            let y_below = current_coordinate.1 + 1;
            if y_below < lowest_position {
                let coordinate_below_straight = (current_coordinate.0, y_below);
                let field_below_straight = self.field.get(&coordinate_below_straight);
                if field_below_straight.is_none() {
                    current_coordinate = coordinate_below_straight;
                    continue;
                }

                let coordinate_below_left = (current_coordinate.0 - 1, y_below);
                let field_below_left = self.field.get(&coordinate_below_left);
                if field_below_left.is_none() {
                    current_coordinate = coordinate_below_left;
                    continue;
                }

                let coordinate_below_right = (current_coordinate.0 + 1, y_below);
                let field_below_right = self.field.get(&coordinate_below_right);
                if field_below_right.is_none() {
                    current_coordinate = coordinate_below_right;
                    continue;
                }

                self.field
                    .try_insert(current_coordinate, Material::Sand)
                    .unwrap_or_else(|err| {
                        panic!("Tried to insert Sand at occupied position {current_coordinate:?} - {err}")
                    });
                return current_coordinate;
            } else {
                self.field
                    .try_insert(current_coordinate, Material::Sand)
                    .unwrap_or_else(|err| {
                        panic!("Tried to insert Sand at occupied position {current_coordinate:?} - {err}")
                    });
                return current_coordinate;
            }
        }
    }
}

pub(crate) type Input = Cave;

pub(crate) fn interpolate(
    from: (usize, usize),
    to: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let min_x = from.0.min(to.0);
    let max_x = from.0.max(to.0);
    let min_y = from.1.min(to.1);
    let max_y = from.1.max(to.1);
    (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| (x, y)))
}

fn parse_input(input: &'static str) -> Input {
    let result = input.lines().map(|line| {
        line.split(" -> ").map(|position_string| {
            let coordinates = position_string.split(',').collect::<Vec<_>>();
            assert_eq!(coordinates.len(), 2);

            (
                coordinates[0].parse::<usize>().unwrap(),
                coordinates[1].parse::<usize>().unwrap(),
            )
        })
    });

    let mut stone_tiles = HashMap::new();

    for lines in result {
        let mut coordinates = lines;
        let mut current_point = coordinates.by_ref().next().unwrap();

        for destination in coordinates {
            for coordinate in interpolate(current_point, destination) {
                stone_tiles.insert(coordinate, Material::Stone);
            }

            current_point = destination;
        }
    }

    Cave { field: stone_tiles }
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(input.clone());
    part_2(input);
}
