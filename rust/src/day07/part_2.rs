use std::collections::HashMap;

use crate::ConsoleLine;

pub(crate) fn part_2(input: &Vec<ConsoleLine>) -> usize {
    const TOTAL_SPACE: usize = 70_000_000;
    const NEEDED_SPACE: usize = 30_000_000;
    let mut current_directory: Vec<&'static str> = vec![];
    let mut directory_sizes = HashMap::<String, usize>::new();

    for line in input {
        match line {
            ConsoleLine::MoveRoot => {
                while current_directory.len() > 1 {
                    current_directory.pop();
                }
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

    let available_space = TOTAL_SPACE - directory_sizes[""];
    let required_space = NEEDED_SPACE - available_space;

    let result = directory_sizes
        .iter()
        .filter_map(|(_directory_name, directory_size)| {
            if *directory_size >= required_space {
                Some(*directory_size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("Part 2: {result}",);

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(&crate::parse_input(crate::INPUT)), 6183184);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(&crate::parse_input(SAMPLE_INPUT)), 24933642);
    }
}
