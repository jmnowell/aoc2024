use std::fs::read_to_string;

// not my code - stolen from someone else
pub fn read_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn parse_report(report: &str) -> Vec<usize> {
    return report.split_ascii_whitespace()
                 .map(|value| { return value.parse::<usize>().unwrap() })
                 .collect();
}

pub fn check_diff(previous: usize, current: usize) -> bool {
    let diff = previous.abs_diff(current);

    if diff < 1 || diff > 3 {
        return false;
    }

    true
}