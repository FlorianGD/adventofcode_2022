use anyhow::{Context, Result};
use num::complex::Complex;
use std::collections::HashSet;

type Moves = Vec<(Complex<isize>, isize)>;
type Grid = HashSet<Complex<isize>>;

pub fn parse_input(_input: &str) -> Result<Moves> {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    input
        .lines()
        .map(|l| {
            let (d, q) = l.split_once(' ').context("wrong input")?;
            let d = match d {
                "R" => Complex::new(1, 0),
                "L" => Complex::new(-1, 0),
                "U" => Complex::new(0, 1),
                "D" => Complex::new(0, -1),
                _ => unreachable!("unknown move"),
            };
            Ok((d, q.parse()?))
        })
        .collect()
}


pub fn part1(input: Moves) -> usize {
    // println!("{:?}", input);
    let mut (current_head, current_tail) = (Complex::new(0,0), Complex::new(0,0));
  let mut grid = Grid::new() ;
  grid.insert(&current_tail);
    for (dir, qty) in input {
      current_head += dir;
      if (current_head - current_tail).l1_norm() > 1 {
        // We move the tail
      }
    }
    0
}
