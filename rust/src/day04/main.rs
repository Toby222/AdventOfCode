const INPUT: &str = include_str!("input.txt");

mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

fn parse_input(input: &'static str) -> Vec<[[u32; 2]; 2]> {
    input
        .lines()
        .map(|line| {
            let pairs = line
                .split(',')
                .take(2)
                .map(|v| {
                    let boundary = v
                        .split('-')
                        .map(|v| v.parse::<u32>().expect("Non-numerical value in input"))
                        .take(2)
                        .collect::<Vec<_>>();
                    assert_eq!(boundary.len(), 2);
                    [boundary[0], boundary[1]]
                })
                .collect::<Vec<_>>();
            assert_eq!(pairs.len(), 2);
            [pairs[0], pairs[1]]
        })
        .collect::<Vec<_>>()
}

pub fn main() {
    let input = parse_input(INPUT);
    part_1(&input);
    part_2(&input);
}
