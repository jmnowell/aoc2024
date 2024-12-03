mod solutions;

use crate::solutions::*;

fn main() {
    let part1_count = part1("input.txt");
    let part2_count = part2("input.txt");

    println!("Part 1 Count: {}", part1_count);
    println!("Part 2 Count: {}", part2_count);
}

#[cfg(test)]
#[test]
fn part1_test() {
    let count = part1("test.txt");
    assert_eq!(count, 2);
}

#[test]
fn part2_test() {
    let count = part2("test.txt");
    assert_eq!(count, 4);
}

#[test]
fn part2_edgecase_test() {
    let count = part2("edge_case.txt");
    assert_eq!(count, 10);
}