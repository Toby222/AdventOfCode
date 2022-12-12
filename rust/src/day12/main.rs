const INPUT: &str = include_str!("input.txt");

mod part_1;
use std::collections::{HashMap, HashSet};

use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Clone, Debug)]
pub(crate) struct ParsedInput {
    pub(crate) grid: HashMap<(usize, usize), u8>,
    pub(crate) start_position: (usize, usize),
    pub(crate) end_position: (usize, usize),
}

fn get_front(frontier: &HashMap<(usize, usize), usize>) -> (usize, usize) {
    let mut items = frontier.iter().collect::<Vec<_>>();
    items.sort_unstable_by(|left, right| left.1.cmp(right.1));
    *items[0].0
}

fn neighbors(
    position: (usize, usize),
    grid: &HashMap<(usize, usize), u8>,
) -> HashSet<(usize, usize)> {
    let (x, y) = position;
    let height = grid[&position];

    [
        (x + 1, y),
        (x.saturating_sub(1), y),
        (x, y + 1),
        (x, y.saturating_sub(1)),
    ]
    .iter()
    .filter_map(|neighbor_position| {
        if *neighbor_position == position {
            None
        } else {
            grid.get(neighbor_position).and_then(|&neighbor_height| {
                (neighbor_height <= height + 1).then_some(*neighbor_position)
            })
        }
    })
    .collect()
}

fn reconstruct_path(
    end_position: (usize, usize),
    start_position: (usize, usize),
    came_from: &HashMap<(usize, usize), Option<(usize, usize)>>,
) -> Option<Vec<(usize, usize)>> {
    let mut current_position = end_position;
    let mut path = vec![];

    while current_position != start_position {
        path.push(current_position);
        if let Some(next) = came_from.get(&current_position) {
            current_position = next.unwrap();
        } else {
            return None;
        }
    }

    Some(path)
}

pub(crate) fn dijkstra(
    grid: &HashMap<(usize, usize), u8>,
    start_position: (usize, usize),
    end_position: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut frontier = HashMap::new();
    frontier.insert(start_position, 0);

    let mut came_from = HashMap::<(_, _), _>::new();
    let mut cost_so_far = HashMap::<(_, _), _>::new();

    came_from.insert(start_position, None);
    cost_so_far.insert(start_position, 0);

    // let mut step = 0;
    while !frontier.is_empty() {
        let current_position = get_front(&frontier);
        // step += 1;
        // println!("Step {step}: {current_position:?}");
        frontier.remove(&current_position);

        if current_position == end_position {
            // println!("Found path after {step} steps");
            break;
        }

        let neighbors = neighbors(current_position, grid);
        // println!("Step {step}: neighbors = {neighbors:?}");
        for neighbor in neighbors {
            let new_cost = cost_so_far[&current_position] + 1;
            let old_cost = cost_so_far.get(&neighbor);

            if old_cost.is_none() || new_cost < *old_cost.unwrap() {
                cost_so_far.insert(neighbor, new_cost);
                frontier.insert(neighbor, new_cost);
                came_from.insert(neighbor, Some(current_position));
            }
        }
    }

    reconstruct_path(end_position, start_position, &came_from)
}

fn parse_input(input: &'static str) -> ParsedInput {
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);

    let matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, byte)| match byte {
                    b'S' => {
                        start_position = (x, y);
                        ((x, y), 0)
                    }
                    b'E' => {
                        end_position = (x, y);
                        ((x, y), b'z' - b'a')
                    }
                    _ => ((x, y), byte - b'a'),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    ParsedInput {
        grid: matrix,
        start_position,
        end_position,
    }
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(&input);
    part_2(&input);
}
