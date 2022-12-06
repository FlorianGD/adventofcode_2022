use anyhow::{Context, Result};
use std::collections::HashSet;

pub fn parse_input(input: &str) -> String {
    input.to_owned()
}

fn find_first_different(input: String, n: usize) -> Result<usize> {
    Ok(input
        .as_bytes()
        .windows(n)
        .enumerate()
        .find(|&(_, p)| HashSet::<&u8>::from_iter(p).len() == n)
        .context("all different")?
        .0
        + n)
}

pub fn part1(input: String) -> usize {
    match find_first_different(input, 4) {
        Ok(n) => n,
        Err(_) => unreachable!("no solution"),
    }
}

pub fn part2(input: String) -> usize {
    match find_first_different(input, 14) {
        Ok(n) => n,
        Err(_) => unreachable!("no solution"),
    }
}
