use anyhow::{Context, Result};
use num::complex::Complex;
use std::collections::HashSet;

type Position = Complex<isize>;
type Moves = Vec<(Position, isize)>;
type Grid = HashSet<Position>;

pub fn parse_input(input: &str) -> Result<Moves> {
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

fn move_tail(head: &Position, tail: &Position) -> Complex<isize> {
    let diff = head - tail;
    match diff.l1_norm() {
        0 | 1 => Complex::new(0, 0),
        2 => {
            if diff.im == 0 {
                Complex::new(diff.re.signum(), 0)
            } else if diff.re == 0 {
                Complex::new(0, diff.im.signum())
            } else {
                // diagonal case, we do not need to move
                Complex::new(0, 0)
            }
        }
        _ => {
            // diagonal case, we need to move towards head diagonally
            Complex::new(diff.re.signum(), diff.im.signum())
        }
    }
}

pub fn part1(input: Moves) -> usize {
    let (mut head, mut tail) = (Complex::new(0, 0), Complex::new(0, 0));
    let mut grid = Grid::new();
    grid.insert(tail.clone());
    for (dir, qty) in input {
        for _ in 0..qty {
            head += dir;
            tail += move_tail(&head, &tail);
            grid.insert(tail.clone());
        }
    }
    grid.len()
}

pub fn part2(input: Moves) -> usize {
    let mut knots = [Complex::<isize>::new(0, 0); 10];
    let mut grid = Grid::new();
    grid.insert(knots[9].clone());
    for (dir, qty) in input {
        for _ in 0..qty {
            knots[0] += dir;
            for i in 1..10 {
                knots[i] += move_tail(&knots[i - 1], &knots[i])
            }
            grid.insert(knots[9].clone());
        }
    }
    grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;

    #[test]
    fn test_move_tail() {
        let tail = Complex::new(0, 0);
        // distance less than 2, don't move
        assert_eq!(move_tail(&Complex::new(1, 0), &tail), tail);
        assert_eq!(move_tail(&Complex::new(0, 1), &tail), tail);
        assert_eq!(move_tail(&Complex::new(-1, 0), &tail), tail);
        assert_eq!(move_tail(&Complex::new(0, -1), &tail), tail);
        // distance 2 same line
        assert_eq!(move_tail(&Complex::new(0, 2), &tail), Complex::new(0, 1));
        assert_eq!(move_tail(&Complex::new(2, 0), &tail), Complex::new(1, 0));
        assert_eq!(move_tail(&Complex::new(0, -2), &tail), Complex::new(0, -1));
        assert_eq!(move_tail(&Complex::new(-2, 0), &tail), Complex::new(-1, 0));
        // just diagonal, don't move
        assert_eq!(move_tail(&Complex::new(1, 1), &tail), tail);
        assert_eq!(move_tail(&Complex::new(-1, -1), &tail), tail);
        assert_eq!(move_tail(&Complex::new(-1, 1), &tail), tail);
        assert_eq!(move_tail(&Complex::new(1, -1), &tail), tail);
        // diagonal further
        assert_eq!(move_tail(&Complex::new(1, 2), &tail), Complex::new(1, 1));
        assert_eq!(move_tail(&Complex::new(2, 1), &tail), Complex::new(1, 1));
        assert_eq!(move_tail(&Complex::new(1, -2), &tail), Complex::new(1, -1));
        assert_eq!(move_tail(&Complex::new(-2, 1), &tail), Complex::new(-1, 1));
        assert_eq!(
            move_tail(&Complex::new(-1, -2), &tail),
            Complex::new(-1, -1)
        );
        assert_eq!(
            move_tail(&Complex::new(-2, -1), &tail),
            Complex::new(-1, -1)
        );
        assert_eq!(move_tail(&Complex::new(-1, 2), &tail), Complex::new(-1, 1));
        assert_eq!(move_tail(&Complex::new(2, -1), &tail), Complex::new(1, -1));
    }

    #[test]
    fn test_part1() -> Result<()> {
        let grid = parse_input(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
",
        )?;
        assert_eq!(part1(grid), 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let grid = parse_input(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
",
        )?;
        assert_eq!(part2(grid), 36);
        Ok(())
    }
}
