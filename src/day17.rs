use anyhow::{anyhow, Result};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(s: char) -> Result<Self> {
        match s {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            s => Err(anyhow!("unknown direction {}", s)),
        }
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Direction>> {
    input.trim().chars().map(Direction::from_char).collect()
}

struct Board {
    top: usize,
    lines: Vec<VecDeque<bool>>,
}

pub struct Block {
    shape: Vec<VecDeque<bool>>,
}

impl Block {
    fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if !self.shape.iter().any(|x| x[0]) {
                    self.shape = self
                        .shape
                        .clone()
                        .into_iter()
                        .map(|mut l| {
                            l.pop_front();
                            l.push_back(false);
                            l
                        })
                        .collect()
                }
            }
            Direction::Right => {
                if !self.shape.iter().any(|x| x[6]) {
                    self.shape = self
                        .shape
                        .clone()
                        .into_iter()
                        .map(|mut l| {
                            l.pop_back();
                            l.push_front(false);
                            l
                        })
                        .collect()
                }
            }
        }
    }
}

// const B1: Block = Block {
//     shape: [[false, false, true, true, true, true, false]],
// };

// const B2: Block = Block {
//     shape: vec![
//         [false, false, false, true, false, false, false],
//         [false, false, true, true, true, false, false],
//         [false, false, false, true, false, false, false],
//     ],
// };

// const B3: Block = Block {
//     shape: vec![
//         [false, false, false, false, true, false, false],
//         [false, false, false, false, true, false, false],
//         [false, false, true, true, true, false, false],
//     ],
// };

// const B4: Block = Block {
//     shape: vec![
//         [false, false, true, false, false, false, false],
//         [false, false, true, false, false, false, false],
//         [false, false, true, false, false, false, false],
//         [false, false, true, false, false, false, false],
//     ],
// };
// const B5: Block = Block {
//     shape: vec![
//         [false, false, true, true, false, false, false],
//         [false, false, true, true, false, false, false],
//     ],
// };

// const BLOCKS: Vec<Block> = vec![B1, B2, B3, B4, B5];
// (
//     [[1, 1, 1, 1]],
//     [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
//     [[0, 0, 1], [0, 0, 1], [1, 1, 1]],
//     [[1], [1], [1], [1]],
//     [[1, 1], [1, 1]],
// );

pub fn part1(directions: Vec<Direction>) -> usize {
    // let mut dir = directions.iter().cycle();
    // let mut blocks = vec![B1, B2, B3, B4, B5].iter().cycle();
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        if let Ok(directions) = parse_input("><") {
            assert_eq!(directions, vec![Direction::Right, Direction::Left])
        } else {
            panic!()
        }
    }

    #[test]
    fn test_shift_single_line() {
        let mut block = Block {
            shape: vec![VecDeque::from([
                false, false, true, true, true, true, false,
            ])],
        };
        block.shift(Direction::Left);
        assert_eq!(
            block.shape,
            vec![VecDeque::from([
                false, true, true, true, true, false, false
            ])]
        );
        let mut block = Block {
            shape: vec![VecDeque::from([
                false, false, true, true, true, true, false,
            ])],
        };
        block.shift(Direction::Right);
        assert_eq!(
            block.shape,
            vec![VecDeque::from([
                false, false, false, true, true, true, true
            ])]
        );
    }

    #[test]
    fn test_shift_reach_side() {
        let mut block = Block {
            shape: vec![VecDeque::from([
                false, false, false, true, true, true, true,
            ])],
        };
        block.shift(Direction::Right);
        assert_eq!(
            block.shape,
            vec![VecDeque::from([
                false, false, false, true, true, true, true,
            ])]
        );
        let mut block = Block {
            shape: vec![VecDeque::from([
                true, true, true, true, false, false, false,
            ])],
        };
        block.shift(Direction::Left);
        assert_eq!(
            block.shape,
            vec![VecDeque::from([
                true, true, true, true, false, false, false,
            ])]
        );
    }

    #[test]
    fn test_shift_plus() {
        let mut plus = Block {
            shape: vec![
                VecDeque::from([false, false, false, true, false, false, false]),
                VecDeque::from([false, false, true, true, true, false, false]),
                VecDeque::from([false, false, false, true, false, false, false]),
            ],
        };
        plus.shift(Direction::Right);
        assert_eq!(
            plus.shape,
            vec![
                VecDeque::from([false, false, false, false, true, false, false]),
                VecDeque::from([false, false, false, true, true, true, false]),
                VecDeque::from([false, false, false, false, true, false, false]),
            ],
        );
        let mut plus = Block {
            shape: vec![
                VecDeque::from([false, false, false, true, false, false, false]),
                VecDeque::from([false, false, true, true, true, false, false]),
                VecDeque::from([false, false, false, true, false, false, false]),
            ],
        };
        plus.shift(Direction::Left);
        assert_eq!(
            plus.shape,
            vec![
                VecDeque::from([false, false, true, false, false, false, false]),
                VecDeque::from([false, true, true, true, false, false, false]),
                VecDeque::from([false, false, true, false, false, false, false]),
            ],
        );
    }
}
