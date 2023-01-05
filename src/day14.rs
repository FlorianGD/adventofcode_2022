use std::collections::HashSet;

use itertools::Itertools;

type Coord = (u32, u32);

const START: Coord = (500, 0);

pub fn parse_input(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    if let Some((x, y)) = coord.split_once(',') {
                        (x.parse().unwrap(), y.parse().unwrap())
                    } else {
                        panic!()
                    }
                })
                .tuple_windows()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let mut range = vec![];
                    if x1 == x2 {
                        if y1 < y2 {
                            for y in y1..=y2 {
                                range.push((x1, y));
                            }
                        } else {
                            for y in y2..=y1 {
                                range.push((x1, y));
                            }
                        }
                    } else {
                        // case y1 == y2
                        if x1 < x2 {
                            for x in x1..=x2 {
                                range.push((x, y1));
                            }
                        } else {
                            for x in x2..=x1 {
                                range.push((x, y1));
                            }
                        }
                    }
                    range
                })
            // .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Boundaries {
    x_min: u32,
    x_max: u32,
    y_max: u32, // no need to check for y_min as it is the source in 0
}

impl Boundaries {
    fn from(walls: &HashSet<Coord>) -> Boundaries {
        let x_max = walls
            .iter()
            .max_by(|&&(k1, _), &&(k2, _)| k1.cmp(&k2))
            .unwrap()
            .0;
        let y_max = walls
            .iter()
            .max_by(|&&(_, k1), &&(_, k2)| k1.cmp(&k2))
            .unwrap()
            .1;
        let x_min = walls
            .iter()
            .min_by(|&&(k1, _), &&(k2, _)| k1.cmp(&k2))
            .unwrap()
            .0;

        Boundaries {
            x_min,
            x_max,
            y_max,
        }
    }

    fn is_outside(&self, (x, y): &Coord) -> bool {
        x < &self.x_min || x > &self.x_max || y > &self.y_max
    }
}

fn pour_sand(walls: &mut HashSet<Coord>, boundaries: &Boundaries) {
    // find the first wall right at the bottom
    let mut coord = START;
    loop {
        if !walls.contains(&(coord.0, coord.1 + 1)) {
            if boundaries.is_outside(&(coord.0, coord.1 + 1)) {
                break;
            }
            coord = (coord.0, coord.1 + 1)
        } else if !walls.contains(&(coord.0 - 1, coord.1 + 1)) {
            if boundaries.is_outside(&(coord.0 - 1, coord.1 + 1)) {
                break;
            }
            coord = (coord.0 - 1, coord.1 + 1)
        } else if !walls.contains(&(coord.0 + 1, coord.1 + 1)) {
            if boundaries.is_outside(&(coord.0 + 1, coord.1 + 1)) {
                break;
            }
            coord = (coord.0 + 1, coord.1 + 1)
        } else {
            walls.insert(coord);
            break;
        }
    }
}

pub fn part1(mut input: HashSet<(u32, u32)>) -> usize {
    let b = Boundaries::from(&input);
    let initial_len = input.len();
    let mut len = input.len();
    loop {
        pour_sand(&mut input, &b);
        if input.len() > len {
            len = input.len()
        } else {
            break;
        }
    }
    input.len() - initial_len
}

fn add_floor(walls: &mut HashSet<(u32, u32)>, b: &Boundaries) {
    let y_max = b.y_max + 2;
    for x in (b.x_min - y_max)..=(b.x_max + y_max) {
        walls.insert((x, y_max));
    }
}

pub fn part2(mut input: HashSet<(u32, u32)>) -> usize {
    let mut b = Boundaries::from(&input);
    add_floor(&mut input, &b);
    b = Boundaries::from(&input);
    let initial_len = input.len();
    let mut len = input.len();
    loop {
        pour_sand(&mut input, &b);
        if input.len() > len {
            len = input.len()
        } else {
            break;
        }
    }
    input.len() - initial_len
}
