fn get_optimal_hold_time(maximum_time: u64, distance_to_beat: u64) -> (u64, u64) {
    let mut minimum_hold_time = None;
    let mut maximum_hold_time = None;
    for time in 0..=maximum_time {
        let distance = time * (maximum_time - time);
        if distance > distance_to_beat && minimum_hold_time.is_none() {
            minimum_hold_time = Some(time);
            maximum_hold_time = Some(maximum_time - time);
            break;
        }
    }
    match (minimum_hold_time, maximum_hold_time) {
        (Some(min), Some(max)) => (min, max),
        _ => unreachable!("Should get a maximum and minimum hold time"),
    }
}

pub(crate) fn part_2(input: &'static str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("Invalid number");
    let distance = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("Invalid number");

    let (minimum_time, maximum_time) = get_optimal_hold_time(time, distance);
    maximum_time - minimum_time + 1
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(SAMPLE_INPUT), 71503);
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_2(crate::INPUT), 36872656);
    }
}
