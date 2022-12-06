#![feature(array_windows)]
#![cfg_attr(test, feature(test))]
#![allow(soft_unstable)]
#![cfg(test)]
extern crate test;

const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1_vec;
mod part_2;
use part_2::part_2_vec;

pub fn main() {
    println!("Part 1: {}", part_1_vec(INPUT));
    println!("Part 2: {}", part_2_vec(INPUT));
}
