use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input<B: BufRead>(buf: B) -> Vec<u64> {
    buf.lines()
        .map(|line| {
            line.expect("Failed to read line.")
                .parse()
                .expect("Failed to parse line input.")
        })
        .collect()
}

fn puzzle_a(nums: &[u64]) -> Option<u64> {
    for a in nums {
        for b in nums {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }
    None
}

fn puzzle_b(nums: &[u64]) -> Option<u64> {
    for a in nums {
        for b in nums {
            for c in nums {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}

fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn main() -> Result<(), &'static str> {
    let buffer = open_file("input").unwrap();
    let puzzle_input = parse_input(buffer);

    println!(
        "Part A:\n{}",
        puzzle_a(&puzzle_input).expect("No solution for A found.")
    );
    println!("--------");
    println!(
        "Part B:\n{}",
        puzzle_b(&puzzle_input).expect("No solution for B found.")
    );

    Ok(())
}
