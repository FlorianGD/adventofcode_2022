use itertools::Itertools;
use std::collections::HashSet;

type Paquet = (char, char, char, char);

pub fn parse_input(input: &str) -> String {
    input.to_owned()
}

pub fn part1(input: String) -> usize {
    input
        .chars()
        .tuple_windows::<Paquet>()
        .enumerate()
        .find(|(_, p)| HashSet::from([p.0, p.1, p.2, p.3]).len() == 4)
        .unwrap()
        .0
        + 4
}

pub fn part2(input: String) -> usize {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|&(_, p)| HashSet::<&u8>::from_iter(p).len() == 14)
        .unwrap()
        .0
        + 14
}
