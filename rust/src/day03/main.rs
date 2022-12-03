const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

pub(crate) fn character_priority(character: u8) -> u64 {
    (match character {
        b'a'..=b'z' => character - b'a' + 1,
        b'A'..=b'Z' => character - b'A' + 27,
        _ => panic!("Invalid character '{character}', no priority"),
    }) as u64
}

pub fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
