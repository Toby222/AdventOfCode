use crate::Shape;

pub(crate) fn part_1(input: &[[Shape; 2]]) {
    let final_score = input.iter().fold(0u64, |acc, [enemy, mine]| {
        acc + u64::from(&mine.play_against(enemy)) + u64::from(mine)
    });
    println!("Part 1: {final_score}");
}
