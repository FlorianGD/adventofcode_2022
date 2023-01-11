use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use num::Complex;

type Map = HashMap<Complex<isize>, bool>;
type Path = Vec<(isize, Complex<isize>)>;

fn draw_board(map: &Map, position: &Complex<isize>, direction: &Complex<isize>) {
    use std::{thread, time};

    let dur = time::Duration::from_millis(100);
    print!("\x1B[2J\x1B[1;1H");
    let max_x = map.iter().map(|(p, _)| p.re).max().unwrap();
    let max_y = map.iter().map(|(p, _)| p.im).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = Complex::new(x, y);
            if &c == position {
                match direction {
                    Complex { re: 0, im: 1 } => print!("v"),
                    Complex { re: 0, im: -1 } => print!("^"),
                    Complex { re: 1, im: 0 } => print!(">"),
                    Complex { re: -1, im: 0 } => print!("<"),
                    _ => unreachable!(),
                }
                continue;
            }
            match map.get(&c) {
                Some(false) => print!("."),
                Some(true) => print!("#"),
                None => print!(" "),
            }
        }
        println!();
    }
    println!();
    thread::sleep(dur);
}

fn parse_map(map: &str) -> Map {
    map.lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(i, c)| -> Option<(Complex<isize>, bool)> {
                    match c {
                        '.' => Some((Complex::new(i as isize, j as isize), false)),
                        '#' => Some((Complex::new(i as isize, j as isize), true)),
                        _ => None,
                    }
                })
        })
        .collect()
}

fn direction(s: &str) -> IResult<&str, Complex<isize>> {
    alt((
        map_res(char('L'), |_| Ok::<_, Error>(Complex::new(0, -1))),
        map_res(char('R'), |_| Ok::<_, Error>(Complex::new(0, 1))),
    ))(s)
}

fn one_instr(s: &str) -> IResult<&str, (isize, Complex<isize>)> {
    tuple((map_res(digit1, |s: &str| s.parse()), direction))(s)
}

fn parse_instr(instr: &str) -> IResult<&str, Path> {
    many1(one_instr)(instr)
}

pub fn parse_input(input: &str) -> Result<(Map, Path)> {
    // use indoc::indoc;
    // let input = indoc! {
    //                                                     "        ...#
    //         .#..
    //         #...
    //         ....
    // ...#.......#
    // ........#...
    // ..#....#....
    // ..........#.
    //         ...#....
    //         .....#..
    //         .#......
    //         ......#

    // 10R5L5R10L4R5L5"};
    if let Some((map, instr)) = input.split_once("\n\n") {
        let map = parse_map(map);
        if let Ok((_, instr)) = parse_instr(instr) {
            Ok((map, instr))
        } else {
            Err(anyhow!("something wrong happened"))
        }
    } else {
        unreachable!()
    }
}

fn score(position: &Complex<isize>, direction: &Complex<isize>) -> isize {
    let facing = match direction {
        Complex { re: 0, im: 1 } => 1,
        Complex { re: 0, im: -1 } => 3,
        Complex { re: 1, im: 0 } => 0,
        Complex { re: -1, im: 0 } => 2,
        _ => unreachable!(),
    };
    1000 * (position.im + 1) + 4 * (position.re + 1) + facing
}

pub fn part1((map, instr): (Map, Path)) -> isize {
    let start = map
        .iter()
        .filter_map(|(Complex { re: i, im: j }, _)| if j == &0 { Some(*i) } else { None })
        .min()
        .unwrap();
    let mut position = Complex::new(start, 0);
    let mut direction = Complex::new(1, 0);
    // draw_board(&map, &position, &direction);
    for (i, r) in instr {
        for _ in 0..i {
            match map.get(&(position + direction)) {
                Some(false) => position += direction,
                Some(true) => {
                    break;
                }
                None => {
                    // wrap around
                    if direction == Complex::new(1, 0) {
                        let new_re = map
                            .iter()
                            .filter_map(|(p, _)| if p.im == position.im { Some(p) } else { None })
                            .min_by(|x, y| x.re.cmp(&y.re))
                            .unwrap();
                        if map[new_re] {
                            // cannot wrap as there is a wall
                            break;
                        } else {
                            position = *new_re;
                        }
                    } else if direction == Complex::new(-1, 0) {
                        let new_re = map
                            .iter()
                            .filter_map(|(p, _)| if p.im == position.im { Some(p) } else { None })
                            .max_by(|x, y| x.re.cmp(&y.re))
                            .unwrap();
                        if map[new_re] {
                            // cannot wrap as there is a wall
                            break;
                        } else {
                            position = *new_re;
                        }
                    } else if direction == Complex::new(0, 1) {
                        let new_im = map
                            .iter()
                            .filter_map(|(p, _)| if p.re == position.re { Some(p) } else { None })
                            .min_by(|x, y| x.im.cmp(&y.im))
                            .unwrap();
                        if map[new_im] {
                            // cannot wrap as there is a wall
                            break;
                        } else {
                            position = *new_im;
                        }
                    } else if direction == Complex::new(0, -1) {
                        let new_im = map
                            .iter()
                            .filter_map(|(p, _)| if p.re == position.re { Some(p) } else { None })
                            .max_by(|x, y| x.im.cmp(&y.im))
                            .unwrap();
                        if map[new_im] {
                            // cannot wrap as there is a wall
                            break;
                        } else {
                            position = *new_im;
                        }
                    }
                }
            }
            // draw_board(&map, &position, &direction);
        }
        direction *= r;
        // draw_board(&map, &position, &direction);
    }
    score(&position, &direction)
}
