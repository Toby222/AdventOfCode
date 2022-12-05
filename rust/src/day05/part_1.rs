pub(crate) fn part_1(input: crate::ChallengeInput) -> String {
    let mut state = input.initial_state;

    for crate::Instruction {
        origin,
        destination,
        amount,
    } in input.instructions
    {
        let length = state[origin - 1].len();
        let (new_state, moved) = state[origin - 1].split_at(length - amount);
        let new_state = new_state.to_vec();
        let mut moved = moved.to_vec();
        moved.reverse();

        state[origin - 1] = new_state;
        state[destination - 1].append(&mut moved);
    }

    state
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(crate::parse_input(crate::INPUT)), "RTGWZTHLD");
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(crate::parse_input(SAMPLE_INPUT)), "CMZ");
    }
}
