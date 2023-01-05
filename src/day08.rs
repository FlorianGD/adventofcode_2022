use anyhow::{anyhow, Result};
use num::complex::Complex;
use std::collections::HashMap;

type Tree = Complex<isize>;
type Grid = HashMap<Tree, u32>;

pub fn parse_input(input: &str) -> Result<Grid> {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars().enumerate().map(move |(i, c)| {
                Ok((
                    Complex::new(i.try_into()?, j.try_into()?),
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("non digit char {}", c))?,
                ))
            })
        })
        .collect()
}

fn is_visible(grid: &Grid, tree: &Tree, height: &u32) -> bool {
    grid.iter()
        .filter(|(k, _)| (k.im < tree.im) && (k.re == tree.re))
        .all(|(_, v)| v < height)
        || grid
            .iter()
            .filter(|(k, _)| (k.im > tree.im) && (k.re == tree.re))
            .all(|(_, v)| v < height)
        || grid
            .iter()
            .filter(|(k, _)| (k.im == tree.im) && (k.re < tree.re))
            .all(|(_, v)| v < height)
        || grid
            .iter()
            .filter(|(k, _)| (k.im == tree.im) && (k.re > tree.re))
            .all(|(_, v)| v < height)
}

pub fn part1(grid: Grid) -> usize {
    grid.iter()
        .filter(|&(k, v)| is_visible(&grid, k, v))
        .count()
}

fn view(grid: &Grid, direction: &Tree, tree: &Tree, height: &u32) -> isize {
    let mut view = 0;
    let mut new_tree = tree + direction;
    loop {
        match grid.get(&new_tree) {
            None => return view,
            Some(h) if h < height => {
                new_tree += direction;
                view += 1;
            }
            Some(_) => return view + 1,
        }
    }
}

pub fn part2(grid: Grid) -> isize {
    const DIRECTIONS: &[Complex<isize>; 4] = &[
        Complex::new(0, 1),
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, -1),
    ];
    grid.iter()
        .map(|(k, v)| {
            DIRECTIONS
                .iter()
                .fold(1, |acc, d| acc * view(&grid, d, k, v))
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;
    const INPUT: &'static str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert!(grid.is_ok());
        if let Ok(g) = grid {
            assert_eq!(g[&Complex::new(0, 0)], 3);
            assert_eq!(g[&Complex::new(4, 4)], 0);
        }
    }

    #[test]
    fn test_is_visible() {
        if let Ok(g) = parse_input(INPUT) {
            assert!(is_visible(&g, &Complex::new(0, 0), &3));
            assert!(is_visible(&g, &Complex::new(4, 4), &0));
            assert!(is_visible(&g, &Complex::new(1, 1), &5));
            assert!(!is_visible(&g, &Complex::new(3, 3), &4));
        }
    }

    #[test]
    fn test_part1() {
        if let Ok(g) = parse_input(INPUT) {
            assert_eq!(part1(g), 21);
        }
    }

    #[test]
    fn test_view_left() {
        let direction = Complex::new(-1 as isize, 0);
        if let Ok(g) = parse_input(INPUT) {
            assert_eq!(view(&g, &direction, &Complex::new(0, 0), &3), 0);
            assert_eq!(view(&g, &direction, &Complex::new(1, 0), &0), 1);
            assert_eq!(view(&g, &direction, &Complex::new(2, 0), &3), 2);
            assert_eq!(view(&g, &direction, &Complex::new(3, 0), &7), 3);
            assert_eq!(view(&g, &direction, &Complex::new(4, 0), &3), 1);
        }
    }
    #[test]
    fn test_view_top() {
        let direction = Complex::new(0 as isize, -1);
        if let Ok(g) = parse_input(INPUT) {
            assert_eq!(view(&g, &direction, &Complex::new(4, 0), &3), 0);
            assert_eq!(view(&g, &direction, &Complex::new(4, 1), &2), 1);
            assert_eq!(view(&g, &direction, &Complex::new(4, 2), &2), 1);
            assert_eq!(view(&g, &direction, &Complex::new(4, 3), &9), 3);
            assert_eq!(view(&g, &direction, &Complex::new(4, 4), &0), 1);
        }
    }

    #[test]
    fn test_part2() {
        if let Ok(g) = parse_input(INPUT) {
            assert_eq!(part2(g), 8);
        }
    }
}
