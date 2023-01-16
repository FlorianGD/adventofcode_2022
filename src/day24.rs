use std::collections::HashMap;

use anyhow::{Context, Result};
use num::Complex;

type Pos = Complex<isize>;

pub fn parse_input(_input: &str) -> Result<(HashMap<Pos, Vec<Pos>>, isize, isize)> {
    use indoc::indoc;
    let input = indoc! { "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#"};
    let hm = input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars().enumerate().filter_map(move |(i, c)| match c {
                '>' => Some((
                    Complex::new(i as isize, j as isize),
                    vec![Complex::new(1, 0)],
                )),
                '<' => Some((
                    Complex::new(i as isize, j as isize),
                    vec![Complex::new(-1, 0)],
                )),
                '^' => Some((
                    Complex::new(i as isize, j as isize),
                    vec![Complex::new(0, -1)],
                )),
                'v' => Some((
                    Complex::new(i as isize, j as isize),
                    vec![Complex::new(0, 1)],
                )),
                // '.' => Some((Complex::new(i as isize, j as isize), vec![])),
                _ => None,
            })
        })
        .collect();
    let dim_y: isize = input.lines().count().try_into()?;
    let dim_x: isize = input
        .lines()
        .next()
        .context("no lines")?
        .chars()
        .count()
        .try_into()?;
    Ok((hm, dim_x, dim_y))
}

pub fn next_state(
    map: &HashMap<Pos, Vec<Pos>>,
    dim_x: isize,
    dim_y: isize,
) -> HashMap<Pos, Vec<Pos>> {
    let mut next_map = HashMap::new();
    for (&pos, directions) in map {
        for direction in directions {
            let new_entry = match direction {
                Complex { re: 1, im: 0 } => {
                    if pos.re == dim_x - 2 {
                        Complex::new(1, pos.im)
                    } else {
                        pos + direction
                    }
                }
                Complex { re: -1, im: 0 } => {
                    if pos.re == 1 {
                        Complex::new(dim_x - 2, pos.im)
                    } else {
                        pos + direction
                    }
                }
                Complex { re: 0, im: 1 } => {
                    if pos.im == dim_y - 2 {
                        Complex::new(pos.re, 1)
                    } else {
                        pos + direction
                    }
                }
                Complex { re: 0, im: -1 } => {
                    if pos.im == 1 {
                        Complex::new(pos.re, dim_y - 2)
                    } else {
                        pos + direction
                    }
                }
                _ => unreachable!(),
            };
            next_map
                .entry(new_entry)
                .and_modify(|e: &mut Vec<Complex<isize>>| e.push(*direction))
                .or_insert(vec![*direction]);
        }
    }
    next_map
}

fn draw_grid(map: &HashMap<Pos, Vec<Pos>>, dim_x: isize, dim_y: isize) {
    println!();
    for y in 0..dim_y {
        for x in 0..dim_x {
            match map.get(&Complex::new(x, y)) {
                Some(d) => {
                    if d.len() > 1 {
                        print!("{}", d.len())
                    } else {
                        match d[0] {
                            Complex { re: 1, im: 0 } => print!(">"),
                            Complex { re: -1, im: 0 } => print!("<"),
                            Complex { re: 0, im: 1 } => print!("v"),
                            Complex { re: 0, im: -1 } => print!("^"),
                            _ => unreachable!(),
                        }
                    }
                }
                None => {
                    if (y == 0 && x != 1)
                        || x == 0
                        || (y == dim_y - 1 && x != dim_x - 2)
                        || x == dim_x - 1
                    {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
            }
        }
        println!();
    }
}

pub fn part1((mut map, dim_x, dim_y): (HashMap<Pos, Vec<Pos>>, isize, isize)) -> isize {
    draw_grid(&map, dim_x, dim_y);
    for _ in 0..10 {
        map = next_state(&map, dim_x, dim_y);
        draw_grid(&map, dim_x, dim_y);
    }
    0
}
