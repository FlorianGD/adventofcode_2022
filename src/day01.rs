use anyhow::{anyhow, Result};

pub fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.parse()
                        .map_err(|e| anyhow!("Failed to convert '{}': {:?}", line, e))
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .map(|block| block.iter().sum::<u32>())
        .max()
        .unwrap()
}

pub fn part2(input: Vec<Vec<u32>>) -> u32 {
    let mut sums: Vec<_> = input
        .iter()
        .map(|block| block.iter().sum::<u32>())
        .collect();
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}
