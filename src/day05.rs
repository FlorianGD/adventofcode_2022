use anyhow::{Context, Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

const NUM_CRATES: u32 = 9;

type Crates = HashMap<u32, Vec<char>>;

#[derive(Debug)]
pub struct Move {
    quantity: u32,
    origin: u32,
    destination: u32,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(movement: &str) -> Result<Self> {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$")?;
        let caps = re.captures(movement).context("error in regex")?;

        let quantity: u32 = caps.get(1).map_or("", |m| m.as_str()).parse()?;
        let origin: u32 = caps.get(2).map_or("", |m| m.as_str()).parse()?;
        let destination: u32 = caps.get(3).map_or("", |m| m.as_str()).parse()?;
        Ok(Move {
            quantity,
            origin,
            destination,
        })
    }
}

fn parse_moves(moves: &str) -> Result<Vec<Move>> {
    moves.lines().map(|m| m.parse()).collect()
}

fn parse_crates(crates: &str) -> Result<Crates> {
    let re = Regex::new(
        r"^(?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   )$",
    )?;
    let mut parsed: Crates = HashMap::new();
    for line in crates.lines() {
        let caps = match re.captures(line) {
            Some(m) => m,
            None => continue,
        };
        for i in 1..=NUM_CRATES {
            match caps.get(i.try_into()?) {
                Some(s) => {
                    let v = parsed.entry(i).or_default();
                    v.push(s.as_str().chars().next().context("no char")?);
                }
                None => continue,
            }
        }
    }
    for i in 1..=NUM_CRATES {
        parsed.get_mut(&i).context("no entry")?.reverse()
    }
    Ok(parsed)
}

pub fn parse_input(input: &str) -> Result<(Crates, Vec<Move>)> {
    let _input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let (crates, moves) = input.split_once("\n\n").context("wrong input")?;
    let moves = parse_moves(moves)?;
    let crates = parse_crates(crates)?;
    Ok((crates, moves))
}

pub fn fallible_part1(mut crates: Crates, moves: Vec<Move>) -> Result<String> {
    for m in moves {
        for _ in 0..m.quantity {
            let v = crates
                .get_mut(&m.origin)
                .context("no entry")?
                .pop()
                .context("empty")?;
            crates.get_mut(&m.destination).context("no entry")?.push(v);
        }
    }
    let mut r = Vec::new();
    for i in 1..=NUM_CRATES {
        r.push(
            crates
                .get_mut(&i)
                .context("no entry")?
                .pop()
                .context("empty")?,
        )
    }
    Ok(r.into_iter().collect())
}

pub fn part1((crates, moves): (Crates, Vec<Move>)) -> String {
    match fallible_part1(crates, moves) {
        Ok(s) => s,
        Err(_) => unreachable!("wrong processing"),
    }
}

fn fallible_part2(mut crates: Crates, moves: Vec<Move>) -> Result<String> {
    for m in moves {
        let len = crates[&m.origin].len();
        let idx: usize = len - m.quantity as usize;
        let stack = crates
            .get_mut(&m.origin)
            .context("no entry")?
            .split_off(idx);
        crates
            .get_mut(&m.destination)
            .context("no entry")?
            .extend(stack.clone());
    }
    let mut r = Vec::new();
    for i in 1..=NUM_CRATES {
        r.push(crates[&i].iter().last().context("empty")?)
    }
    Ok(r.into_iter().collect())
}

pub fn part2((crates, moves): (Crates, Vec<Move>)) -> String {
    match fallible_part2(crates, moves) {
        Ok(s) => s,
        Err(_) => unreachable!("wrong processing"),
    }
}
