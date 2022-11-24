use anyhow::{anyhow, Result};

pub fn parse_input(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|e| anyhow!("Failed to convert '{}': {:?}", line, e))
        })
        .collect()
}

pub fn part1(input: Vec<u32>) -> u32 {
    let mut total = 0;
    for (x, y) in input.iter().zip(input[1..].iter()) {
        if x < y {
            total += 1;
        }
    }
    total
}

pub fn part2(input: Vec<u32>) -> u32 {
    let mut strides: Vec<u32> = Vec::new();
    for i in 0..input.len() - 2 {
        strides.push(input[i..=i + 2].iter().sum());
    }
    part1(strides)
}
