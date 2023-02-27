use std::collections::HashMap;

use anyhow::{Error, Result};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use num::Complex;

type Map = HashMap<Complex<isize>, bool>;
type Path = Vec<(isize, Option<Complex<isize>>)>;

fn _draw_board(map: &Map, position: &Complex<isize>, direction: &Complex<isize>) {
    use std::{thread, time};

    let dur = time::Duration::from_millis(100);
    // print!("\x1B[2J\x1B[1;1H");
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

fn one_instr(s: &str) -> IResult<&str, (isize, Option<Complex<isize>>)> {
    tuple((map_res(digit1, |s: &str| s.parse()), opt(direction)))(s)
}

fn parse_instr(instr: &str) -> IResult<&str, Path> {
    many1(one_instr)(instr)
}

pub fn parse_input(input: &str) -> Result<(Map, Path)> {
    // use indoc::indoc;
    // let input = indoc! {
    // "        ...#
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
    //         ......#.

    // 10R5L5R10L4R5L5"};
    if let Some((map, instr)) = input.split_once("\n\n") {
        let map = parse_map(map);
        if let Ok((_, instr)) = parse_instr(instr) {
            return Ok((map, instr));
        }
    }
    unreachable!()
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
    // _draw_board(&map, &position, &direction);
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
            // _draw_board(&map, &position, &direction);
        }
        if let Some(d) = r {
            direction *= d;
        }
        // _draw_board(&map, &position, &direction);
    }
    score(&position, &direction)
}

// I do not have a general solution for all folds possible, I hardcode the one for my
// input, where the shape is (each number is a face of the cube)
//  12
//  3
// 45
// 6
// It is NOT the same as the input (:skull:), so it won't work with the example...
fn get_side(c: Complex<isize>, square_size: isize) -> Option<u8> {
    match ((c.re / square_size) % 3, (c.im / square_size) % 4) {
        (1, 0) => Some(1),
        (2, 0) => Some(2),
        (1, 1) => Some(3),
        (0, 2) => Some(4),
        (1, 2) => Some(5),
        (0, 3) => Some(6),
        _ => None,
    }
}

pub fn part2((map, instr): (Map, Path)) -> isize {
    let start = map
        .iter()
        .filter_map(|(Complex { re: i, im: j }, _)| if j == &0 { Some(*i) } else { None })
        .min()
        .unwrap();
    let mut position = Complex::new(start, 0);
    let mut direction = Complex::new(1, 0);
    let square_size = (map.keys().map(|p| p.re).max().unwrap() + 1) / 3;
    for (i, r) in instr {
        for _ in 0..i {
            match map.get(&(position + direction)) {
                Some(false) => position += direction,
                Some(true) => {
                    break;
                }
                None => {
                    match (get_side(position, square_size), direction) {
                        (Some(1), Complex { re: -1, im: 0 }) => {
                            // 1< => 4>
                            let new_pos =
                                Complex::new(0, 3 * square_size - (position.im % square_size) - 1);
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(1, 0);
                                assert_eq!(get_side(position, square_size), Some(4))
                            }
                        }
                        (Some(1), Complex { re: 0, im: -1 }) => {
                            // 1^ => 6>
                            let new_pos =
                                Complex::new(0, 3 * square_size + (position.im % square_size));
                            if map[&new_pos] {
                                // cannot wrap as there is a wall
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(1, 0);
                                assert_eq!(get_side(position, square_size), Some(6))
                            }
                        }
                        (Some(2), Complex { re: 0, im: -1 }) => {
                            // 2^ => 6^
                            let new_pos =
                                Complex::new(position.re % square_size, 4 * square_size - 1);
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, -1);
                                assert_eq!(get_side(position, square_size), Some(6))
                            }
                        }
                        (Some(2), Complex { re: 1, im: 0 }) => {
                            // 2> => 5<
                            let new_pos = Complex::new(
                                2 * square_size - 1,
                                3 * square_size - (position.im % square_size) - 1,
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(-1, 0);
                                assert_eq!(get_side(position, square_size), Some(5))
                            }
                        }
                        (Some(2), Complex { re: 0, im: 1 }) => {
                            // 2v => 3<
                            let new_pos = Complex::new(
                                2 * square_size - 1,
                                square_size + (position.im % square_size),
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(-1, 0);
                                assert_eq!(get_side(position, square_size), Some(3))
                            }
                        }
                        (Some(3), Complex { re: -1, im: 0 }) => {
                            // 3< => 4v
                            let new_pos = Complex::new(position.im % square_size, 2 * square_size);
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, -1);
                                assert_eq!(get_side(position, square_size), Some(4))
                            }
                        }
                        (Some(3), Complex { re: 1, im: 0 }) => {
                            // 3> => 2^
                            let new_pos = Complex::new(
                                2 * square_size + (position.im % square_size),
                                square_size - 1,
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, -1);
                                assert_eq!(get_side(position, square_size), Some(2))
                            }
                        }
                        (Some(4), Complex { re: 0, im: -1 }) => {
                            // 4^ => 3>
                            let new_pos = Complex::new(
                                square_size,
                                square_size + (position.re % square_size),
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(1, 0);
                                assert_eq!(get_side(position, square_size), Some(3))
                            }
                        }
                        (Some(4), Complex { re: -1, im: 0 }) => {
                            // 4< => 1>
                            let new_pos = Complex::new(
                                square_size,
                                square_size - (position.im % square_size) - 1,
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(1, 0);
                                assert_eq!(get_side(position, square_size), Some(1))
                            }
                        }
                        (Some(5), Complex { re: 1, im: 0 }) => {
                            // 5> => 2<
                            let new_pos = Complex::new(
                                3 * square_size - 1,
                                square_size - (position.im % square_size) - 1,
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, -1);
                                assert_eq!(get_side(position, square_size), Some(2))
                            }
                        }
                        (Some(5), Complex { re: 0, im: 1 }) => {
                            // 5v => 6<
                            let new_pos = Complex::new(
                                square_size - 1,
                                3 * square_size + (position.re % square_size),
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(-1, 0);
                                assert_eq!(get_side(position, square_size), Some(6))
                            }
                        }
                        (Some(6), Complex { re: -1, im: 0 }) => {
                            // 6< => 1v
                            let new_pos =
                                Complex::new(square_size + (position.im % square_size), 0);
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, 1);
                                assert_eq!(get_side(position, square_size), Some(1))
                            }
                        }
                        (Some(6), Complex { re: 1, im: 0 }) => {
                            // 6> => 5^
                            let new_pos = Complex::new(
                                square_size + (position.im % square_size),
                                3 * square_size - 1,
                            );
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, -1);
                                assert_eq!(get_side(position, square_size), Some(5))
                            }
                        }
                        (Some(6), Complex { re: 0, im: 1 }) => {
                            // 6v => 2v
                            let new_pos =
                                Complex::new(2 * square_size + (position.re % square_size), 0);
                            if map[&new_pos] {
                                break;
                            } else {
                                position = new_pos;
                                direction = Complex::new(0, 1);
                                assert_eq!(get_side(position, square_size), Some(2))
                            }
                        }
                        (x, y) => {
                            println!("position {:?}", position);
                            println!("square_size = {square_size}");
                            println!("{:?} {:?}", x, y);
                            panic!()
                        }
                    }
                }
            }
        }
        if let Some(d) = r {
            direction *= d;
        }
    }
    score(&position, &direction)
}
