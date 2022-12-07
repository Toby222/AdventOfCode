use std::collections::HashMap;

use crate::ConsoleLine;

pub(crate) fn part_1(input: &Vec<ConsoleLine>) -> usize {
    let mut current_directory: Vec<&'static str> = vec![];
    let mut directory_sizes = HashMap::<String, usize>::new();

    for line in input {
        match line {
            ConsoleLine::MoveRoot => {
                current_directory.clear();
                current_directory.push("");
            }
            ConsoleLine::MoveUp => {
                current_directory.pop();
            }
            ConsoleLine::MoveDown(new_dir) => {
                current_directory.push(new_dir);
            }
            ConsoleLine::List => {}
            ConsoleLine::ListOutputDir(_dir_name) => {}
            ConsoleLine::ListOutputFile(_file_name, file_size) => {
                for length in 0..=current_directory.len() {
                    let parent_directory = current_directory[0..length].join("/");

                    if let Err(mut a) = directory_sizes.try_insert(parent_directory, *file_size) {
                        let t = a.entry.get_mut();
                        *t += a.value;
                    };
                }
            }
        }
    }
    let result = directory_sizes
        .iter()
        .filter_map(|(_, v)| if *v <= 100_000 { Some(v) } else { None })
        .sum();

    println!("Part 1: {result}",);

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::parse_input(crate::INPUT)), 1084134);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&crate::parse_input(SAMPLE_INPUT)), 95437);
    }
}
