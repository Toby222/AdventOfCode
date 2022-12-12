use crate::Op;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

pub(crate) fn part_2(input: &[Op]) -> [String; SCREEN_HEIGHT] {
    let mut cycle = 0i64;
    let mut register_x = 1;
    let mut current_line = 0;

    let mut result = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];

    for op in input {
        for _ in 0..i64::from(op) {
            match cycle {
                40 => current_line += 1,
                80 => current_line += 1,
                120 => current_line += 1,
                160 => current_line += 1,
                200 => current_line += 1,
                _ => {}
            }
            cycle += 1;

            let corrected_x = (cycle - 1) - register_x - (current_line * SCREEN_WIDTH) as i64;
            // println!(
            //     "cycle = {cycle}, register_x = {register_x}, current_line = {current_line}, corrected_x = {corrected_x}",
            // );

            result[current_line].push(match corrected_x {
                -1 | 0 | 1 => '#',
                _ => '.',
            });
        }

        match op {
            Op::Noop => {}
            Op::AddX(x) => {
                register_x += x;
            }
        }
    }

    println!("Part2:\n{}", result.join("\n"));

    result
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_with_solution() {
        assert_eq!(
            super::part_2(&crate::parse_input(crate::INPUT)),
            [
                "###...##..####.####.#..#.#..#.###..#..#.",
                "#..#.#..#....#.#....#..#.#..#.#..#.#.#..",
                "#..#.#......#..###..####.#..#.#..#.##...",
                "###..#.##..#...#....#..#.#..#.###..#.#..",
                "#.#..#..#.#....#....#..#.#..#.#.#..#.#..",
                "#..#..###.####.####.#..#..##..#..#.#..#.",
            ]
        );
    }

    #[test]
    fn test_with_sample_solution() {
        assert_eq!(
            super::part_2(&crate::parse_input(SAMPLE_INPUT)),
            [
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
        );
    }
}
