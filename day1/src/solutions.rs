use std::fs::read_to_string;

// not my code - stolen from someone else
fn read_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn prepare_file(path: &str) -> (Vec<usize>, Vec<usize>) {
    let unsorted_paired_ids = read_lines(path);
    let mut left_ids: Vec<usize> = Vec::new();
    let mut right_ids: Vec<usize> = Vec::new();

    // extract the numbers from each row
    for pairs in unsorted_paired_ids {
        let mut split_pair = pairs.split_ascii_whitespace();
        left_ids.push(split_pair.next().as_deref().unwrap().parse::<usize>().unwrap());
        right_ids.push(split_pair.next().as_deref().unwrap().parse::<usize>().unwrap());
    }

    left_ids.sort();
    right_ids.sort();
    assert_eq!(left_ids.len(), right_ids.len());

    return (left_ids, right_ids);
}

pub fn part1(path: &str) -> usize {
    let (left_ids, right_ids) = prepare_file(path);
    let mut diff: Vec<usize> = Vec::new();

    for (idx, left_value) in left_ids.iter().enumerate() {
        let right_value = right_ids[idx];

        if *left_value > right_value {
            diff.push(*left_value - right_value);
        } else {
            diff.push(right_value - *left_value);
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