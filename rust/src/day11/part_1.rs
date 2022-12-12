use crate::Monkey;

const ROUND_COUNT: usize = 20;

pub(crate) fn part_1(input: &[Monkey]) -> u64 {
    let mut monkey_state = input.to_owned();
    let mut times_inspected = vec![0; monkey_state.len()];
    for _ in 0..ROUND_COUNT {
        for monkey_idx in 0..monkey_state.len() {
            while let Some(worry) = monkey_state[monkey_idx].items.pop() {
                let worry = monkey_state[monkey_idx].operation.perform(worry) / 3;

                let throw_to = if worry % monkey_state[monkey_idx].test.divisible_by == 0 {
                    monkey_state[monkey_idx].test.if_true
                } else {
                    monkey_state[monkey_idx].test.if_false
                };

                monkey_state[throw_to].items.push(worry);

                times_inspected[monkey_idx] += 1;
            }
        }
    }

    times_inspected.sort_unstable();
    times_inspected.reverse();
    let result = times_inspected[0] * times_inspected[1];

    println!("Part 1: {result}");

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::parse_input(crate::INPUT)), 58786);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&crate::parse_input(SAMPLE_INPUT)), 10605);
    }
}
