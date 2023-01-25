use std::collections::HashMap;

use anyhow::{Context, Result};
use num::{integer::lcm, Complex};
use pathfinding::directed::dijkstra::dijkstra;

type Pos = Complex<isize>;

pub fn parse_input(input: &str) -> Result<(HashMap<Pos, Vec<Pos>>, isize, isize)> {
    // use indoc::indoc;
    // let input = indoc! { "#.######
    // #>>.<^<#
    // #.<..<<#
    // #>v.><>#
    // #<^v^^>#
    // ######.#"};
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

fn next_state(map: &HashMap<Pos, Vec<Pos>>, dim_x: isize, dim_y: isize) -> HashMap<Pos, Vec<Pos>> {
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

fn _draw_grid(map: &HashMap<Pos, Vec<Pos>>, dim_x: isize, dim_y: isize) {
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

fn possible_next_steps(
    current: &Pos,
    minute: isize,
    maps: &Vec<HashMap<Pos, Vec<Pos>>>,
    dim_x: isize,
    dim_y: isize,
) -> Vec<(isize, Pos)> {
    let neighbors = [
        Complex::new(0, 0), // wait
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, 1),
        Complex::new(0, -1),
    ];
    let len = maps.len() as isize;
    let next_minute = ((minute + 1) % len) as usize;
    neighbors
        .into_iter()
        .filter_map(|n| {
            let maybe_new_pos = current + n;
            if maybe_new_pos.re <= 0 {
                None
            } else if maybe_new_pos.re >= dim_x - 1 {
                None
            } else if maybe_new_pos.im < 0 || maybe_new_pos.im == 0 && maybe_new_pos.re > 1 {
                None
            } else if maybe_new_pos.im > dim_y - 1
                || maybe_new_pos.im == dim_y - 1 && maybe_new_pos.re != dim_x - 2
            {
                None
            } else {
                match maps[next_minute].get(&maybe_new_pos) {
                    Some(_) => None,
                    None => Some((next_minute as isize, maybe_new_pos)),
                }
            }
        })
        .collect()
}

pub fn part1((mut map, dim_x, dim_y): (HashMap<Pos, Vec<Pos>>, isize, isize)) -> usize {
    let mut maps = vec![];
    // compute all possible states
    for _ in 0..lcm(dim_x - 2, dim_y - 2) {
        // print!("\n##### Minute {} ####", i);
        let new_map = next_state(&map, dim_x, dim_y);
        // draw_grid(&map, dim_x, dim_y);
        maps.push(map);
        map = new_map;
    }

    let start = Complex::new(1, 0);
    let end = Complex::new(dim_x - 2, dim_y - 1);

    let result = dijkstra(
        &(0, start),
        |(m, c)| {
            possible_next_steps(c, *m, &maps, dim_x, dim_y)
                .into_iter()
                .map(|p| (p, 1))
        },
        |p| p.1 == end,
    );

    if let Some((_, l)) = result {
        return l;
    }
    unreachable!()
}

pub fn part2((mut map, dim_x, dim_y): (HashMap<Pos, Vec<Pos>>, isize, isize)) -> usize {
    let mut maps = vec![];
    // compute all possible states
    for _ in 0..lcm(dim_x - 2, dim_y - 2) {
        // println!("##### Minute {} ####", i);
        let new_map = next_state(&map, dim_x, dim_y);
        // draw_grid(&map, dim_x, dim_y);
        maps.push(map);
        map = new_map;
    }

    let start = Complex::new(1, 0);
    let end = Complex::new(dim_x - 2, dim_y - 1);

    let r1 = dijkstra(
        &(0, start),
        |(m, c)| {
            possible_next_steps(c, *m, &maps, dim_x, dim_y)
                .into_iter()
                .map(|p| (p, 1))
        },
        |p| p.1 == end,
    );

    if let Some((r1, l1)) = r1 {
        // we go back to the start
        let last_minute = r1[r1.len() - 1].0;
        let end = Complex::new(1, 0);
        let start = Complex::new(dim_x - 2, dim_y - 1);

        let r2 = dijkstra(
            &(last_minute, start),
            |(m, c)| {
                possible_next_steps(c, *m, &maps, dim_x, dim_y)
                    .into_iter()
                    .map(|p| (p, 1))
            },
            |p| p.1 == end,
        );
        if let Some((r2, l2)) = r2 {
            // then to the end again

            let last_minute = r2[r2.len() - 1].0;
            let start = Complex::new(1, 0);
            let end = Complex::new(dim_x - 2, dim_y - 1);

            let r3 = dijkstra(
                &(last_minute, start),
                |(m, c)| {
                    possible_next_steps(c, *m, &maps, dim_x, dim_y)
                        .into_iter()
                        .map(|p| (p, 1))
                },
                |p| p.1 == end,
            );
            if let Some((_, l3)) = r3 {
                return l1 + l2 + l3;
            }
        }
    }
    unreachable!()
}
