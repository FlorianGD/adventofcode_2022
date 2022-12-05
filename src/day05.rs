use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;

const LEN: u32 = 9;
#[derive(Debug)]
pub struct Move {
    quantity: u32,
    origin: u32,
    destination: u32,
}

impl Move {
    pub fn from_str(movement: &str) -> Result<Self> {
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
    moves.lines().map(Move::from_str).collect()
}

fn parse_crates(crates: &str) -> Result<HashMap<u32, Vec<char>>> {
    let re = Regex::new(
        r"^(?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   )$",
        // r"^(?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   )$",
    )?;
    let mut parsed: HashMap<u32, Vec<char>> = HashMap::new();
    for line in crates.lines() {
        let caps = match re.captures(line) {
            Some(m) => m,
            None => continue,
        };
        for i in 1..=LEN {
            match caps.get(i.try_into()?) {
                Some(s) => {
                    let v = parsed.entry(i as u32).or_insert(Vec::new());
                    v.push(s.as_str().chars().next().context("no char")?);
                }
                None => continue,
            }
        }
    }
    for i in 1..=LEN {
        parsed.get_mut(&i).context("no entry")?.reverse()
    }
    Ok(parsed)
}

pub fn parse_input(input: &str) -> Result<(HashMap<u32, Vec<char>>, Vec<Move>)> {
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

pub fn part1((mut crates, moves): (HashMap<u32, Vec<char>>, Vec<Move>)) -> String {
    for m in moves {
        for _ in 0..m.quantity {
            let v = crates.get_mut(&m.origin).unwrap().pop().unwrap();
            crates.get_mut(&m.destination).unwrap().push(v);
        }
    }
    let mut r = Vec::new();
    for i in 1..=LEN {
        r.push(crates.get_mut(&i).unwrap().pop().unwrap())
    }
    r.into_iter().collect()
}

pub fn part2((mut crates, moves): (HashMap<u32, Vec<char>>, Vec<Move>)) -> String {
    for m in moves {
        let mut stack = Vec::new();
        for _ in 0..m.quantity {
            let v = crates.get_mut(&m.origin).unwrap().pop().unwrap();
            stack.push(v);
        }
        stack.reverse();
        crates
            .get_mut(&m.destination)
            .unwrap()
            .extend(stack.clone());
    }
    let mut r = Vec::new();
    for i in 1..=LEN {
        r.push(crates.get_mut(&i).unwrap().pop().unwrap())
    }
    r.into_iter().collect()
}
