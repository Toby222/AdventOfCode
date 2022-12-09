use std::collections::HashSet;

use crate::Move;
const TAIL_LENGTH: usize = 9;

#[derive(Clone, Debug)]
pub(crate) struct Rope<const N: usize = 1> {
    head: (i64, i64),
    tails: [(i64, i64); N],
    tail_history: HashSet<(i64, i64)>,
}
impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope {
            head: (0, 0),
            tails: [(0, 0); N],
            tail_history: HashSet::new(),
        }
    }
    fn pos_diff(&self, index: usize) -> (i64, i64) {
        let head = if index == 0 {
            self.head
        } else {
            self.tails[index - 1]
        };
        let tail = self.tails[index];
        (tail.0 - head.0, tail.1 - head.1)
    }
    fn execute(&mut self, Move(dir, count): &Move) {
        let dir = <(i64, i64)>::from(*dir);
        for _ in 0..*count {
            self.head.0 += dir.0;
            self.head.1 += dir.1;

            for i in 0..N {
                let tail_movement = tail_update(self.pos_diff(i));
                self.tails[i].0 += tail_movement.0;
                self.tails[i].1 += tail_movement.1;
            }
            self.tail_history.insert(self.tails[N - 1]);
        }
    }
}

fn tail_update(diff: (i64, i64)) -> (i64, i64) {
    let reaction = (-diff.0.signum(), -diff.1.signum());
    let distance = diff.0.abs() + diff.1.abs();
    let has_zero_component = diff.0.abs() == 0 || diff.1.abs() == 0;
    match (has_zero_component, distance) {
        (true, 2) | (_, 3..) => reaction,
        _ => (0, 0),
    }
}

pub(crate) fn part_2(input: &Vec<Move>) -> usize {
    let mut rope = Rope::<TAIL_LENGTH>::new();

    for head_movement in input {
        rope.execute(head_movement);
    }

    let result = rope.tail_history.len();

    println!("Part 2: {result}");

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(&crate::parse_input(crate::INPUT)), 2458);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(&crate::parse_input(SAMPLE_INPUT)), 1);
    }
}
