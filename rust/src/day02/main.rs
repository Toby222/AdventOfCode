const INPUT: &str = include_str!("input.txt");

mod part_1;
use std::str::FromStr;

use part_1::part_1;
mod part_2;
use part_2::part_2;

#[derive(Clone, Copy)]
pub(crate) enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
pub(crate) enum GameResult {
    Win,
    Tie,
    Loss,
}

impl Shape {
    fn play_against(&self, other: &Shape) -> GameResult {
        match self {
            Shape::Rock => match other {
                Shape::Rock => GameResult::Tie,
                Shape::Paper => GameResult::Loss,
                Shape::Scissors => GameResult::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => GameResult::Win,
                Shape::Paper => GameResult::Tie,
                Shape::Scissors => GameResult::Loss,
            },
            Shape::Scissors => match other {
                Shape::Rock => GameResult::Loss,
                Shape::Paper => GameResult::Win,
                Shape::Scissors => GameResult::Tie,
            },
        }
    }

    fn get_response_for_result(&self, result: &GameResult) -> Self {
        match self {
            Shape::Rock => match result {
                GameResult::Tie => Shape::Rock,
                GameResult::Loss => Shape::Scissors,
                GameResult::Win => Shape::Paper,
            },
            Shape::Paper => match result {
                GameResult::Win => Shape::Scissors,
                GameResult::Tie => Shape::Paper,
                GameResult::Loss => Shape::Rock,
            },
            Shape::Scissors => match result {
                GameResult::Loss => Shape::Paper,
                GameResult::Win => Shape::Rock,
                GameResult::Tie => Shape::Scissors,
            },
        }
    }
}

impl From<&Shape> for u64 {
    fn from(shape: &Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl From<&GameResult> for u64 {
    fn from(game_result: &GameResult) -> Self {
        match game_result {
            GameResult::Win => 6,
            GameResult::Tie => 3,
            GameResult::Loss => 0,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParseShapeError;

impl FromStr for Shape {
    type Err = ParseShapeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 1);
        let Some(character) = s.bytes().next() else {
            return Err(ParseShapeError);
        };

        match character {
            b'A' | b'X' => Ok(Shape::Rock),
            b'B' | b'Y' => Ok(Shape::Paper),
            b'C' | b'Z' => Ok(Shape::Scissors),
            _ => Err(ParseShapeError),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParseGameResultError;

impl FromStr for GameResult {
    type Err = ParseGameResultError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 1);
        let Some(character) = s.bytes().next() else {
            return Err(ParseGameResultError);
        };

        match character {
            b'X' => Ok(GameResult::Loss),
            b'Y' => Ok(GameResult::Tie),
            b'Z' => Ok(GameResult::Win),
            _ => Err(ParseGameResultError),
        }
    }
}

pub fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
