#![feature(const_mut_refs)]
#![feature(array_windows)]
#![feature(test)]
extern crate test;

const INPUT: &str = include_str!("input.txt");

use solutions::*;

mod part_1;
mod part_2;
mod solutions;

pub fn main() {
    println!(
        "Part 1: {}",
        solve_nicopap_vertesians_nodeps_const::<4>(INPUT)
    );
    println!(
        "Part 2: {}",
        solve_nicopap_vertesians_nodeps_const::<14>(INPUT)
    );
}
