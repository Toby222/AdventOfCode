fn get_optimal_hold_time(maximum_time: u16, distance_to_beat: u16) -> (u16, u16) {
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

pub(crate) fn part_1(input: &'static str) -> u64 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<u16>().expect("Invalid number"));
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse::<u16>().expect("Invalid number"));

    let races = times
        .zip(distances)
        .map(|(time, distance)| {
            let hold_times = get_optimal_hold_time(time, distance);
            (hold_times.1 - hold_times.0 + 1) as u64
        })
        .product();

    races
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(SAMPLE_INPUT), 288)
    }

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::INPUT), 393120);
    }
}
