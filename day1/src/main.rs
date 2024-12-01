mod solutions;
mod commons;

use crate::solutions::*;

fn main() {
    let part1_ans = part1("input.txt");
    let part2_ans = part2("input.txt");

    println!("Part 1 Answer: {}", part1_ans);
    println!("Part 2 Answer: {}", part2_ans);
}


#[cfg(test)]
#[test]
fn part1_test() {
    let test_ans = part1("test.txt");
    assert_eq!(test_ans, 11);
}

#[test]
fn part2_test() {
    let test_ans = part2("test.txt");
    assert_eq!(test_ans, 31);
}