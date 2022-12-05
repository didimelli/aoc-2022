use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Match {
    opponent: char,
    player: char,
}

impl Match {
    fn is_winning(&self) -> bool {
        winning().contains(&self)
    }
    fn draws(&self) -> bool {
        draws().contains(&self)
    }

    fn points(&self) -> u32 {
        let mut points = 0;
        if self.is_winning() {
            points += 6;
        } else if self.draws() {
            points += 3;
        }
        match self.player {
            'X' => points += 1,
            'Y' => points += 2,
            'Z' => points += 3,
            _ => panic!("There shouldn't be other characters!"),
        }
        // println!("ok got those points {} in this match {:?}", points, self);
        points
    }

    fn points_part_two(&self) -> u32 {
        let mut points = 0;
        match self.player {
            'X' => match self.opponent {
                'A' => points += 0 + 3,
                'B' => points += 0 + 1,
                'C' => points += 0 + 2,
                _ => panic!("There shouldn't be other characters!"),
            },
            'Y' => match self.opponent {
                'A' => points += 3 + 1,
                'B' => points += 3 + 2,
                'C' => points += 3 + 3,
                _ => panic!("There shouldn't be other characters!"),
            },
            'Z' => match self.opponent {
                'A' => points += 6 + 2,
                'B' => points += 6 + 3,
                'C' => points += 6 + 1,
                _ => panic!("There shouldn't be other characters!"),
            },
            _ => panic!("There shouldn't be other characters!"),
        };
        points
    }
}
// (1 for Rock, 2 for Paper, and 3 for Scissors)
fn winning() -> HashSet<Match> {
    [
        Match {
            opponent: 'A', // rock
            player: 'Y',   // paper
        },
        Match {
            opponent: 'B', //paper
            player: 'Z',   // scissors
        },
        Match {
            opponent: 'C', // scissors
            player: 'X',   // rock
        },
    ]
    .into()
}
fn draws() -> HashSet<Match> {
    [
        Match {
            opponent: 'A',
            player: 'X',
        },
        Match {
            opponent: 'B',
            player: 'Y',
        },
        Match {
            opponent: 'C',
            player: 'Z',
        },
    ]
    .into()
}

fn parse_input(path: &str) -> Vec<Match> {
    let input = read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            return Match {
                opponent: split.next().unwrap().chars().next().unwrap(),
                player: split.next().unwrap().chars().next().unwrap(),
            };
        })
        .collect()
}

fn part_one(input: &[Match]) -> u32 {
    input.iter().fold(0, |sum, x| sum + x.points())
}

fn part_two(input: &[Match]) -> u32 {
    input.iter().fold(0, |sum, x| sum + x.points_part_two())
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = parse_input("input/day02_input.txt");
    let sol1: u32 = part_one(&input);
    let sol2: u32 = part_two(&input);

    (Solution::U32(sol1), Solution::U32(sol2))
}

#[test]
fn test_part_one() {
    let input = parse_input("input/day02_test.txt");
    assert_eq!(part_one(&input), 15);
}

#[test]
fn test_part_two() {
    let input = parse_input("input/day02_test.txt");
    assert_eq!(part_two(&input), 12);
}
