#![feature(map_try_insert)]
const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Debug)]
pub(crate) enum ConsoleLine {
    MoveRoot,
    MoveUp,
    MoveDown(&'static str),
    List,
    ListOutputDir(&'static str),
    ListOutputFile(&'static str, usize),
}

#[derive(Debug)]
pub(crate) struct ConsoleLineParseError;

impl ConsoleLine {
    fn from_str(s: &'static str) -> Result<Self, ConsoleLineParseError> {
        let result = match s.starts_with('$') {
            true => match &s[2..4] {
                "cd" => match s.chars().nth(5) {
                    Some('/') => Ok(ConsoleLine::MoveRoot),
                    Some('.') => Ok(ConsoleLine::MoveUp),
                    Some(_) => Ok(ConsoleLine::MoveDown(&s[5..])),
                    None => Err(ConsoleLineParseError),
                },
                "ls" => Ok(ConsoleLine::List),
                _ => Err(ConsoleLineParseError),
            },
            false => match s.chars().next() {
                Some(char) => match char.is_numeric() {
                    true => {
                        let split_index = s.char_indices().find(|char| char.1 == ' ');
                        match split_index {
                            Some(split_index) => {
                                let (file_size, file_name) = s.split_at(split_index.0);
                                // Remove leading ' '
                                let file_name = &file_name[1..];
                                let file_size = file_size.parse::<usize>();

                                match file_size {
                                    Ok(file_size) => {
                                        Ok(ConsoleLine::ListOutputFile(file_name, file_size))
                                    }
                                    Err(_) => Err(ConsoleLineParseError),
                                }
                            }
                            None => Err(ConsoleLineParseError),
                        }
                    }
                    false => Ok(ConsoleLine::ListOutputDir(&s[4..])),
                },
                None => Err(ConsoleLineParseError),
            },
        };
        if let Ok(result) = result {
            // println!("parsed: {result:?}");
            Ok(result)
        } else {
            result
        }
    }
}

fn parse_input(input: &'static str) -> Vec<ConsoleLine> {
    input
        .lines()
        .map(|line| ConsoleLine::from_str(line).expect("Invalid input"))
        .collect()
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(&input);
    part_2(&input);
}
