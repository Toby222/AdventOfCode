use crate::Monkey;

const ROUND_COUNT: usize = 10_000;

pub(crate) fn part_2(input: &[Monkey]) -> u64 {
    let mut monkey_state = input.to_owned();
    let mut times_inspected = vec![0; monkey_state.len()];

    let lcm = monkey_state
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<u64>();

    for _ in 0..ROUND_COUNT {
        for monkey_idx in 0..monkey_state.len() {
            while let Some(worry) = monkey_state[monkey_idx].items.pop() {
                let worry = monkey_state[monkey_idx].operation.perform(worry);

                let throw_to = if worry % monkey_state[monkey_idx].test.divisible_by == 0 {
                    monkey_state[monkey_idx].test.if_true
                } else {
                    monkey_state[monkey_idx].test.if_false
                };

                monkey_state[throw_to].items.push(worry % lcm);

                times_inspected[monkey_idx] += 1;
            }
        }
    }

    times_inspected.sort_unstable();
    times_inspected.reverse();
    let result = times_inspected[0] * times_inspected[1];

    println!("Part 2: {result}");

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(
            super::part_2(&crate::parse_input(crate::INPUT)),
            14952185856
        );
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_2(&crate::parse_input(SAMPLE_INPUT)), 2713310158);
    }
}
