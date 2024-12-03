mod commons;

use crate::solutions::commons::*;

pub fn part1(path: &str) -> usize {
    let (left_ids, right_ids) = prepare_file(path);
    let mut diff: Vec<usize> = Vec::new();

    for (idx, left_value) in left_ids.iter().enumerate() {
        let right_value = right_ids[idx];
        let left_value = *left_value;

        if left_value > right_value {
            diff.push(left_value - right_value);
        } else {
            diff.push(right_value - left_value);
        }
    }

    return diff.iter().sum::<usize>();
}

pub fn part2(path: &str) -> usize {
    let (left_ids, right_ids) = prepare_file(path);
    let mut sim_score: Vec<usize> = Vec::new();

    for left in left_ids {
        let mut occurance_count: usize = 0;

        for right in &right_ids {
            if *right == left {
                occurance_count += 1;
            }
        }

        sim_score.push(occurance_count * left);
    }

    return sim_score.iter().sum::<usize>();
}