use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Move {
  quantity: u32, 
  origin: u32, 
  desrination: u32
}
impl Move {
  pub fn from_str(move: &str) -> Result<Self> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$")?;
    let caps = re.captures(move)? ;

    let quantity: u32 = caps.get(1).map_or("", |m| m.as_str()).parse()?;
    let origin: u32 = caps.get(2).map_or("", |m| m.as_str()).parse()?;
    let destination : u32 = caps.get(3).map_or("", |m| m.as_str()).parse()?;
    Ok(Move { quantity, origin, destination })
  }
}
fn parse_moves(moves: &str) -> Result<Vec<Move>> {
  moves.lines().map(Move::from_str)?.collect()
}

fn parse_crates(crates: &str) -> Result<HashMap<u32, Vec<char>>> {
  // TODO : Change it for the whole input
  let re = Regex::new(r"(?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   ) (?:\[([A-Z])\]|   )");
  let mut parsed = HashMap::new();
for line in crates {
  let caps = re.captures(line)?;
  for i in 1..=3 {
    match caps.get(i) {
      Some(s) => {
        let v = *parsed.entries(i).or_insert(Vec::new());
        v.
         } 
    }
  }
}
}

fn parse_crates(crates: &str)
pub fn parse_input(_input: &str) -> Result<(HashMap<u32, Vec<char>>, Vec<Move>)> {
  let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
  let (crates, moves) = input.split_once("\n\n").context("wrong input")?;
  let moves = parse_moves(moves)? ;
  Ok(vec! )
}