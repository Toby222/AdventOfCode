pub(crate) fn part_1(input: &'static str) -> usize {
    let mut visible = vec![vec![false; input.lines().next().unwrap().len()]; input.lines().count()];

    // Left-to-right
    {
        for (line_idx, line) in input.lines().enumerate() {
            let mut biggest_char_in_line = '\0';
            for (char_idx, char) in line.char_indices() {
                if char > biggest_char_in_line {
                    visible[line_idx][char_idx] = true;
                    biggest_char_in_line = char;
                }
            }
        }
    }

    // Right-to-left
    {
        for (line_idx, line) in input.lines().enumerate() {
            let mut biggest_char_in_line = '\0';
            for (char_idx, char) in line.char_indices().rev() {
                if char > biggest_char_in_line {
                    visible[line_idx][char_idx] = true;
                    biggest_char_in_line = char;
                }
            }
        }
    }

    // Top-to-bottom
    {
        let mut biggest_char_in_column = vec!['\0'; input.lines().next().unwrap().len()];

        for (line_idx, line) in input.lines().enumerate() {
            for (char_idx, char) in line.char_indices() {
                if char > biggest_char_in_column[char_idx] {
                    visible[line_idx][char_idx] = true;
                    biggest_char_in_column[char_idx] = char;
                }
            }
        }
    }

    // Bottom-to-top
    {
        let mut biggest_char_in_column = vec!['\0'; input.lines().next().unwrap().len()];

        for (line_idx, line) in input.lines().enumerate().collect::<Vec<_>>().iter().rev() {
            for (char_idx, char) in line.char_indices() {
                if char > biggest_char_in_column[char_idx] {
                    visible[*line_idx][char_idx] = true;
                    biggest_char_in_column[char_idx] = char;
                }
            }
        }
    }

    let result = visible.iter().flatten().filter(|v| **v).count();

    println!("Part 1: {result}");

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 1676);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 21);
    }
}
