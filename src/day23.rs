use std::collections::{HashSet, VecDeque};

use num::Complex;

type Pos = Complex<isize>;
type Elves = HashSet<Pos>;
type VecElves = Vec<Pos>;
type Neighbors = (Pos, Pos, Pos);

pub fn parse_input(input: &str) -> Elves {
    // use indoc::indoc;
    // let input = indoc! { "..............
    // ..............
    // .......#......
    // .....###.#....
    // ...#...#.#....
    // ....#...##....
    // ...#.###......
    // ...##.#.##....
    // ....#..#......
    // ..............
    // ..............
    // ..............
    // "};
    input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars().enumerate().filter_map(move |(i, c)| match c {
                '#' => Some(Complex::new(i as isize, j as isize)),
                _ => None,
            })
        })
        .collect()
}

fn has_neighbor(e: &Pos, elves: &Elves) -> bool {
    for d in [
        Complex::new(-1, -1),
        Complex::new(-1, 0),
        Complex::new(-1, 1),
        Complex::new(0, -1),
        Complex::new(0, 1),
        Complex::new(1, -1),
        Complex::new(1, 0),
        Complex::new(1, 1),
    ] {
        if elves.contains(&(e + d)) {
            return true;
        }
    }
    return false;
}

fn propose_moves(vec_elves: &VecElves, elves: &Elves, neighbors: &VecDeque<Neighbors>) -> VecElves {
    vec_elves
        .iter()
        .map(|e| {
            for d in neighbors {
                if !has_neighbor(e, elves) {
                    return *e;
                } else if !elves.contains(&(e + d.0))
                    && !elves.contains(&(e + d.1))
                    && !elves.contains(&(e + d.2))
                {
                    return e + match (d.0.re == d.1.re, d.0.im == d.1.im) {
                        (true, false) => Complex::new(d.0.re, 0),
                        (false, true) => Complex::new(0, d.0.im),
                        _ => unreachable!(),
                    };
                }
            }
            *e
        })
        .collect()
}

fn round(elves: &Elves, neighbors: &mut VecDeque<Neighbors>) -> Elves {
    let vec_elves = elves.iter().cloned().collect();
    let proposed = propose_moves(&vec_elves, elves, neighbors);
    neighbors.rotate_left(1);
    elves
        .into_iter()
        .zip(proposed.iter())
        .map(|(initial, prop)| {
            if proposed.iter().filter(|&x| x == prop).count() > 1 {
                *initial
            } else {
                *prop
            }
        })
        .collect()
}

fn _draw_elves(elves: &Elves) {
    let x_min = elves
        .iter()
        .min_by(|x, y| x.re.cmp(&y.re))
        .unwrap()
        .re
        .min(0);
    let x_max = elves
        .iter()
        .max_by(|x, y| x.re.cmp(&y.re))
        .unwrap()
        .re
        .max(10);
    let y_min = elves
        .iter()
        .min_by(|x, y| x.im.cmp(&y.im))
        .unwrap()
        .im
        .min(0);
    let y_max = elves
        .iter()
        .max_by(|x, y| x.im.cmp(&y.im))
        .unwrap()
        .im
        .max(10);
    for j in y_min..=y_max {
        for i in x_min..=x_max {
            if elves.contains(&Complex::new(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn score(elves: &Elves) -> isize {
    let x_min = elves.iter().min_by(|x, y| x.re.cmp(&y.re)).unwrap().re;
    let x_max = elves.iter().max_by(|x, y| x.re.cmp(&y.re)).unwrap().re;
    let y_min = elves.iter().min_by(|x, y| x.im.cmp(&y.im)).unwrap().im;
    let y_max = elves.iter().max_by(|x, y| x.im.cmp(&y.im)).unwrap().im;
    (x_max + 1 - x_min) * (y_max + 1 - y_min) - elves.len() as isize
}

pub fn part1(elves: Elves) -> isize {
    let mut neighbors: VecDeque<Neighbors> = VecDeque::from([
        (
            Complex::new(-1, -1),
            Complex::new(0, -1),
            Complex::new(1, -1),
        ),
        (Complex::new(-1, 1), Complex::new(0, 1), Complex::new(1, 1)),
        (
            Complex::new(-1, -1),
            Complex::new(-1, 0),
            Complex::new(-1, 1),
        ),
        (Complex::new(1, -1), Complex::new(1, 0), Complex::new(1, 1)),
    ]);
    // _draw_elves(&elves);
    let mut elves = elves;
    for _ in 0..10 {
        let new_elves = round(&elves, &mut neighbors);
        // _draw_elves(&new_elves);
        if elves == new_elves {
            // none moved, the process is over
            break;
        }
        elves = new_elves;
    }
    score(&elves)
}

pub fn part2(elves: Elves) -> isize {
    let mut neighbors: VecDeque<Neighbors> = VecDeque::from([
        (
            Complex::new(-1, -1),
            Complex::new(0, -1),
            Complex::new(1, -1),
        ),
        (Complex::new(-1, 1), Complex::new(0, 1), Complex::new(1, 1)),
        (
            Complex::new(-1, -1),
            Complex::new(-1, 0),
            Complex::new(-1, 1),
        ),
        (Complex::new(1, -1), Complex::new(1, 0), Complex::new(1, 1)),
    ]);
    // _draw_elves(&elves);
    let mut elves = elves;
    let mut i = 1;

    loop {
        let new_elves = round(&elves, &mut neighbors);
        // _draw_elves(&new_elves);
        if elves == new_elves {
            // none moved, the process is over
            return i;
        }
        elves = new_elves;
        i += 1;
    }
}
