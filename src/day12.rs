use anyhow::{Error, Result};
use pathfinding::prelude::{bfs, Matrix};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    map: Matrix<i8>,
    start: Pos,
    goal: Pos,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Map> {
        let start = grid
            .lines()
            .enumerate()
            .find_map(|(i, r)| {
                if let Some(j) =
                    r.chars()
                        .enumerate()
                        .find_map(|(j, c)| if c == 'S' { Some(j) } else { None })
                {
                    Some(Pos(i, j))
                } else {
                    None
                }
            })
            .unwrap();
        let goal = grid
            .lines()
            .enumerate()
            .find_map(|(i, r)| {
                if let Some(j) =
                    r.chars()
                        .enumerate()
                        .find_map(|(j, c)| if c == 'E' { Some(j) } else { None })
                {
                    Some(Pos(i, j))
                } else {
                    None
                }
            })
            .unwrap();

        let map = Matrix::from_rows(grid.lines().map(|row| {
            row.chars().map(|c| match c {
                'a'..='z' => c as i8 - 'a' as i8,
                'S' => 0,
                'E' => 25,
                _ => unreachable!("unknown char found"),
            })
        }))?;
        Ok(Map { map, start, goal })
    }
}

impl Map {
    fn successors(&self, Pos(i, j): &Pos) -> Vec<Pos> {
        let current_val = self.map[(*i, *j)];
        self.map
            .neighbours((*i, *j), false)
            .filter_map(|(i_, j_)| {
                if self.map[(i_, j_)] - current_val <= 1 {
                    Some(Pos(i_, j_))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn parse_input(input: &str) -> Result<Map> {
    input.parse()
}

pub fn part1(map: Map) -> isize {
    let result = bfs(&map.start, |p| map.successors(p), |p| *p == map.goal);
    result.unwrap().len() as isize - 1
}

pub fn part2(map: Map) -> isize {
    let mut starts = vec![];
    for i in 0..map.map.rows {
        for j in 0..map.map.columns {
            if map.map[(i, j)] == 0 {
                starts.push(Pos(i, j));
            }
        }
    }
    bfs(
        &map.goal,
        |&Pos(i, j)| {
            let current_val = map.map[(i, j)];
            map.map
                .neighbours((i, j), false)
                .filter_map(|(i_, j_)| {
                    if current_val - map.map[(i_, j_)] <= 1 {
                        Some(Pos(i_, j_))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Pos>>()
        },
        |&Pos(i, j)| map.map[(i, j)] == 0,
    )
    .unwrap()
    .len() as isize
        - 1
}

#[cfg(test)]
mod test {
    use super::*;
    use pathfinding::prelude::Matrix;

    #[test]
    fn test_map_parse() {
        let input = "aSb
cdE";
        if let Ok(map) = input.parse::<Map>() {
            assert_eq!(
                map,
                Map {
                    map: Matrix::from_vec(2, 3, vec![0i8, 0, 1, 2, 3, 25]).unwrap(),
                    start: Pos(0, 1),
                    goal: Pos(1, 2)
                }
            );
        }
    }

    #[test]
    fn test_map_successors() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        if let Ok(map) = parse_input(input) {
            assert_eq!(map.start, Pos(0, 0));
            assert_eq!(map.successors(&map.start), vec![Pos(0, 1), Pos(1, 0)]);
        }
    }

    #[test]
    fn test_part1() {
        let input = "SabcdefghijklmnopqrstuvwxyzE";
        if let Ok(map) = parse_input(input) {
            assert_eq!(part1(map), 27);
        }
    }

    #[test]
    fn test_part2() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        if let Ok(map) = parse_input(input) {
            assert_eq!(part2(map), 29);
        }
    }
}
