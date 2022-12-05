use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

fn parse_input(path: &str) -> Vec<Vec<u64>> {
    let input = read_to_string(path).unwrap();
    input
        .split("\n\n")
        .into_iter()
        .map(|s| s.lines().map(|item| item.parse().unwrap()).collect())
        .collect()
}

fn part_one(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|sub| sub.iter().sum::<u64>())
        .max()
        .unwrap()
}

fn part_two(input: &[Vec<u64>]) -> u64 {
    let mut asd = input
        .iter()
        .map(|sub| sub.iter().sum::<u64>())
        .collect::<Vec<u64>>();
    asd.sort();
    let len = asd.len();
    asd[len - 3..len].iter().sum()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = parse_input("input/day01_input.txt");
    let sol1: u64 = part_one(&input);
    let sol2: u64 = part_two(&input);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_part_one() {
    let input = parse_input("input/day01_test.txt");
    assert_eq!(part_one(&input), 24000);
}

#[test]
fn test_part_two() {
    let input = parse_input("input/day01_test.txt");
    assert_eq!(part_two(&input), 45000);
}
