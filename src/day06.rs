use itertools::Itertools;
use std::collections::HashSet;

type Paquet = (char, char, char, char);

pub fn parse_input(input: &str) -> String {
    input.to_own
}

pub fn part1(input: String) -> usize {
    input
        .chars()
        .tuple_windows::<Paquet>()
        .enumerate()
        .filter(|(i, p)| HashSet::from([p.0, p.1, p.2, p.3]).len() == 4)
        .next()
        .unwrap()
        .0
}
