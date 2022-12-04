use itertools::Itertools;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<(String, String)> {
    //  let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    //jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    //PmmdzqPrVvPwwTWBwg
    //wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    //ttgJtRGJQctTZtZT
    //CrZsJsPPZsGzwwsLwLmpwMDw";
    input
        .lines()
        .map(|line| {
            let half = line.len() / 2; // OK because ASCII
            (line[..half].to_string(), line[half..].to_string())
        })
        .collect()
}

pub fn part1(input: Vec<(String, String)>) -> u32 {
    input
        .into_iter()
        .map(|(l, r)| {
            let set: HashSet<char> = l.chars().collect();
            for c in r.chars() {
                if set.contains(&c) {
                    return c;
                }
            }
            unreachable!("no common char");
        })
        .map(|c| match c {
            'a'..='z' => (c as u32 - 'a' as u32) + 1,
            'A'..='Z' => (c as u32 - 'A' as u32) + 26 + 1,
            _ => unreachable!("wrong input"),
        })
        .sum()
}

pub fn parse_input_p2(input: &str) -> Vec<char> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let s: HashSet<char> = chunk.next().unwrap().chars().collect();
            let s2: HashSet<char> = chunk.next().unwrap().chars().collect();
            for c in chunk.next().unwrap().chars() {
                if (s.intersection(&s2)).contains(&c) {
                    return c;
                }
            }
            unreachable!("no common char");
        })
        .collect()
}

pub fn part2(input: Vec<char>) -> u32 {
    input
        .into_iter()
        .map(|c| match c {
            'a'..='z' => (c as u32 - 'a' as u32) + 1,
            'A'..='Z' => (c as u32 - 'A' as u32) + 26 + 1,
            _ => unreachable!("wrong input"),
        })
        .sum()
}
