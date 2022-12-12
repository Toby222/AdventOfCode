const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Op {
    Add(u64),
    Mul(u64),
    Square,
}
impl Op {
    fn perform(&self, worry: u64) -> u64 {
        match self {
            Op::Add(a) => a + worry,
            Op::Mul(a) => a * worry,
            Op::Square => worry.pow(2),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Test {
    pub divisible_by: u64,
    pub if_true: usize,
    pub if_false: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Monkey {
    pub id: usize,
    pub items: Vec<u64>,
    pub operation: Op,
    pub test: Test,
}

pub(crate) fn parse_input(input: &'static str) -> Vec<Monkey> {
    let normalized_line_endings = input.replace('\r', "");
    let monkeys = normalized_line_endings.split("\n\n");

    let mut result = Vec::new();
    for monkey in monkeys {
        let mut chars = monkey.chars();

        for _ in 0.."Monkey ".len() {
            chars.next();
        }
        let id_string = chars
            .by_ref()
            .take_while(|char| !matches!(char, ':'))
            .collect::<String>();

        for _ in 0.."\n  Starting items: ".len() {
            chars.next();
        }
        let starting_items_string = chars
            .by_ref()
            .take_while(|char| !matches!(char, '\n'))
            .collect::<String>();

        for _ in 0.."  Operation: new = old ".len() {
            chars.next();
        }
        let operation_string = chars
            .by_ref()
            .take_while(|char| !matches!(char, '\n'))
            .collect::<String>();

        for _ in 0.."  Test: divisible by ".len() {
            chars.next();
        }
        let divisible_by_string = chars
            .by_ref()
            .take_while(|char| matches!(char, '-' | '0'..='9'))
            .collect::<String>();

        for _ in 0.."    If true: throw to monkey ".len() {
            chars.next();
        }
        let if_true_string = chars
            .by_ref()
            .take_while(|char| char.is_numeric())
            .collect::<String>();

        for _ in 0.."    If false: throw to monkey ".len() {
            chars.next();
        }
        let if_false_string = chars
            .by_ref()
            .take_while(|char| char.is_numeric())
            .collect::<String>();

        let id = id_string.parse().unwrap();
        let starting_items = starting_items_string
            .split(", ")
            .map(|num_string| num_string.parse().unwrap())
            .collect();
        let operation = if operation_string.starts_with('+') {
            Op::Add(
                operation_string
                    .chars()
                    .skip(2)
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        } else if operation_string.chars().nth(2) == Some('o') {
            Op::Square
        } else {
            Op::Mul(
                operation_string
                    .chars()
                    .skip(2)
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        };
        let divisible_by = divisible_by_string.parse().unwrap();
        let if_true = if_true_string.parse().expect("Non-numeric if_true?");
        let if_false = if_false_string.parse().unwrap();
        let test = Test {
            divisible_by,
            if_true,
            if_false,
        };
        result.push(Monkey {
            id,
            items: starting_items,
            operation,
            test,
        })
    }

    result
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(&input);
    part_2(&input);
}
