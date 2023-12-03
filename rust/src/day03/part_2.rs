use std::collections::HashMap;

#[derive(Debug)]
struct PartNumber {
    number: u64,
    adjacent_gear: usize,
}

pub(crate) fn part_2(input: &'static str) -> u64 {
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

            let mut gears = vec![];
            // This `line.len()` assumes the input is square
            for y in y.saturating_sub(1)..=usize::min(y + 1, line.len() - 1) {
                for x in (search_index + number_start).saturating_sub(1)
                    ..=usize::min(search_index + number_end, line.len())
                {
                    let idx = y * (line.len() + 1) + x;
                    let char = input.as_bytes()[idx] as char;
                    if matches!(char, '*') {
                        gears.push(idx)
                    }
                }
            }

            part_numbers.extend(gears.iter().map(|&gear| PartNumber {
                number,
                adjacent_gear: gear,
            }));
            search_index += number_end;
        }
    }

    let mut groups = HashMap::new();

    for part_number in part_numbers {
        let Some(group) = groups.get_mut(&part_number.adjacent_gear) else {
            groups.insert(part_number.adjacent_gear, vec![part_number]);
            continue;
        };
        group.push(part_number);
    }

    groups
        .values()
        .filter(|group| group.len() == 2)
        .map(|group| group[0].number * group[1].number)
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 467835);
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 80703636);
    }
}
