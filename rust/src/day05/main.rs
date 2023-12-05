const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

fn main() -> Result<(), std::num::ParseIntError> {
    println!("{}", part_1(INPUT)?);
    println!("{}", part_2(INPUT)?);
    Ok(())
}
