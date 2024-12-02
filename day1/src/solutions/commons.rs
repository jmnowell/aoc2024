use std::fs::read_to_string;

// not my code - stolen from someone else
fn read_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn prepare_file(path: &str) -> (Vec<usize>, Vec<usize>) {
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