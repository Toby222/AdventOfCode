use crate::{GameResult, Shape};

pub(crate) fn part_2(input: &[(Shape, GameResult)]) {
    let final_score = input
        .iter()
        .fold(0u64, |acc, (enemy_play, desired_result)| {
            acc + u64::from(&enemy_play.get_response_for_result(desired_result))
                + u64::from(desired_result)
        });
    println!("Part 2: {final_score}");
}
