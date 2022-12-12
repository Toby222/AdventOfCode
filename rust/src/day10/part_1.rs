use crate::Op;

pub(crate) fn part_1(input: &[Op]) -> i64 {
    let mut cycle = 0i64;
    let mut register_x = 1;

    let mut values = [0; 6];
    for op in input {
        for _ in 0..i64::from(op) {
            cycle += 1;
            match cycle {
                20 => values[0] = register_x * cycle,
                60 => values[1] = register_x * cycle,
                100 => values[2] = register_x * cycle,
                140 => values[3] = register_x * cycle,
                180 => values[4] = register_x * cycle,
                220 => values[5] = register_x * cycle,
                _ => {}
            }
        }

        match op {
            Op::Noop => {}
            Op::AddX(x) => {
                register_x += x;
            }
        }
    }

    let result = values.iter().sum();
    println!("Part 1: {result}");

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(super::part_1(&crate::parse_input(crate::INPUT)), 14860);
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(super::part_1(&crate::parse_input(SAMPLE_INPUT)), 13140);
    }
}
