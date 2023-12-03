use std::iter::Sum;

#[derive(Debug)]
struct PartNumber(u64);

impl<'a> Sum<&'a PartNumber> for u64 {
    fn sum<I: Iterator<Item = &'a PartNumber>>(iter: I) -> Self {
        iter.fold(0, |acc, x| acc + x.0)
    }
}

pub(crate) fn part_1(input: &'static str) -> u64 {
    let mut part_numbers = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut search_index = 0;
        loop {
            let search_line = &line[search_index..];
            if search_line.is_empty() {
                break;
            }
            let Some(number_start) = search_line.find(|c| char::is_ascii_digit(&c)) else {
                break;
            };
            let number_end = search_line[number_start..]
                .find(|c| !char::is_ascii_digit(&c))
                .unwrap_or(search_line.len() - number_start)
                + number_start;

            let number = search_line[number_start..number_end]
                .parse()
                .expect("Should be a valid number");

            // This `line.len()` assumes the input is square
            let mut has_symbol_neighbor = false;
            'symbol_search_loop: for y in y.saturating_sub(1)..=usize::min(y + 1, line.len() - 1) {
                for x in (search_index + number_start).saturating_sub(1)
                    ..=usize::min(search_index + number_end, line.len())
                {
                    let idx = y * (line.len() + 1) + x;
                    let char = input.as_bytes()[idx] as char;
                    if !matches!(char, '0'..='9' | '.' | '\n') {
                        has_symbol_neighbor = true;
                        break 'symbol_search_loop;
                    }
                }
            }

            if has_symbol_neighbor {
                part_numbers.push(PartNumber(number));
            }
            search_index += number_end;
        }
    }

    part_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 4361)
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 539590);
    }
}
