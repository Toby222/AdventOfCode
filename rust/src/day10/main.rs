const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Op {
    Noop,
    AddX(i64),
}

// Represents the cycle count
impl From<Op> for i64 {
    fn from(op: Op) -> Self {
        match op {
            Op::Noop => 1,
            Op::AddX(_) => 2,
        }
    }
}
impl From<&Op> for i64 {
    fn from(op: &Op) -> Self {
        match op {
            Op::Noop => 1,
            Op::AddX(_) => 2,
        }
    }
}

impl From<&'static str> for Op {
    fn from(s: &'static str) -> Self {
        let bits = s.split(' ').collect::<Vec<_>>();
        match bits[0] {
            "noop" => Self::Noop,
            "addx" => Self::AddX(
                bits[1]
                    .parse::<i64>()
                    .expect("Non-numerical addx instruction"),
            ),
            _ => unreachable!("Unknown operation {:?}", bits),
        }
    }
}

fn parse_input(input: &'static str) -> Vec<Op> {
    input.lines().map(Op::from).collect()
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(&input);
    part_2(&input);
}
