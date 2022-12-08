use anyhow::{anyhow, Result};
use num::complex::Complex;
use std::collections::HashMap;

type Grid = HashMap<Complex<usize>, u32>;

pub fn parse_input(input: &str) -> Result<Grid> {
    let _input = "30373
25512
65332
33549
35390
";
    input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars().enumerate().map(move |(i, c)| {
                Ok((
                    Complex::new(i, j),
                    c.to_digit(10).ok_or(anyhow!("non digit char {}", c))?,
                ))
            })
        })
        .collect()
}


fn is_visible_from_one_side(side: &Grid, height: & u32) -> bool {
  side.iter().all(|(_, v)| v < height)
}
fn is_visible(grid: &Grid, tree: Complex<usize>) -> bool {
  let height = &grid[&tree];
   grid.iter().filter(|(k,_)| (k.im < tree.im) && (k.re == tree.re)).all(|(_, v)| v < height) ||
   grid.iter().filter(|(k,_)| (k.im > tree.im) && (k.re == tree.re)).all(|(_, v)| v < height) ||
   grid.iter().filter(|(k,_)| (k.im == tree.im) && (k.re < tree.re)).all(|(_, v)| v < height) ||
  grid.iter().filter(|(k,_)| (k.im == tree.im) && (k.re > tree.re)).all(|(_, v)| v < height) 
  
}

pub fn part1(grid: Grid) -> usize {
    // println!("{:?}", grid);
  grid.iter().filter(|&(k, _)| is_visible(&grid, *k)).count()
    
}
