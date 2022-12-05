#![feature(iter_array_chunks)]

const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Debug, Clone)]
pub(crate) struct Instruction {
    origin: usize,
    destination: usize,
    amount: usize,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.amount, self.origin, self.destination
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ChallengeInput {
    initial_state: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_input(input: &'static str) -> ChallengeInput {
    let (unparsed_instructions, unparsed_initial_state) = input
        .lines()
        .partition::<Vec<_>, _>(|line| line.starts_with("move"));

    let mut initial_state = vec![];
    unparsed_initial_state
        .iter()
        .rev()
        .filter(|line| line.contains('['))
        .for_each(|line| {
            let mut array_chunks = line.chars().array_chunks::<4>();
            let mut idx = 0;
            loop {
                let Some([lbracket, char, _rbracket, _space]) = array_chunks.next() else {
                    break;
                };
                for _ in 0..usize::saturating_sub(1 + 1 + idx, initial_state.len()) {
                    initial_state.push(vec![]);
                }

                if lbracket != '[' {
                    idx += 1;
                    continue;
                }

                initial_state[idx].push(char);
                idx += 1;
            }
            let remainder = array_chunks.into_remainder().unwrap().collect::<Vec<_>>();
            let last_char = remainder[1];
            drop(remainder);
            if last_char != ' ' {
                let length = initial_state.len();
                initial_state[length - 1].push(last_char);
            }
        });

    let instructions = unparsed_instructions
        .iter()
        .map(|instruction| {
            // println!("instruction={instruction}");
            let mut chars = instruction.chars().peekable();
            loop {
                if match chars.peek() {
                    None => true,
                    Some(char) => char.is_numeric(),
                } {
                    break;
                }
                chars.next();
            }
            let mut amount = String::new();
            loop {
                if match chars.peek() {
                    None => true,
                    Some(char) => !char.is_numeric(),
                } {
                    break;
                }
                amount.push(chars.next().unwrap());
            }
            // println!("Amount: {amount}");
            loop {
                if match chars.peek() {
                    None => true,
                    Some(char) => char.is_numeric(),
                } {
                    break;
                }
                chars.next();
            }
            let mut origin = String::new();
            loop {
                if match chars.peek() {
                    None => true,
                    Some(char) => !char.is_numeric(),
                } {
                    break;
                }
                origin.push(chars.next().unwrap());
            }
            loop {
                if match chars.peek() {
                    None => true,
                    Some(char) => char.is_numeric(),
                } {
                    break;
                }
                chars.next();
            }
            let mut destination = String::new();
            loop {
                if match chars.peek() {
                    None => true,
                    Some(char) => !char.is_numeric(),
                } {
                    break;
                }
                destination.push(chars.next().unwrap());
            }

            Instruction {
                origin: origin.parse().expect("Non-numeric input encountered"),
                destination: destination.parse().expect("Non-numeric input encountered"),
                amount: amount.parse().expect("Non-numeric input encountered"),
            }
        })
        .collect();

    ChallengeInput {
        initial_state,
        instructions,
    }
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(input.clone());
    part_2(input);
}
