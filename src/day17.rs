use std::fmt::Display;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Clone)]
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

pub fn parse_input(_input: &str) -> Result<Vec<Direction>> {
    let mut input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    #[cfg(test)]
    {
        input = _input;
    }
    input.trim().chars().map(Direction::from_char).collect()
}

#[derive(Debug, Clone)]
struct Board {
    lines: Vec<u8>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for line in self.lines.iter().rev() {
            let r = format!("{:07b}", line).replace("0", ".").replace("1", "#");
            writeln!(f, "{r}");
        })
    }
}
impl Board {
    fn new() -> Self {
        Board { lines: vec![127] }
    }

    fn scan(&self, block_len: usize, offset: usize) -> Vec<&u8> {
        if block_len > offset + 1 {
            self.lines.iter().rev().take(offset + 1).collect()
        } else {
            self.lines
                .iter()
                .rev()
                .skip(offset + 1 - block_len)
                .take(block_len)
                .collect()
        }
    }

    fn can_move_block_down(&self, block: &Block, offset: usize) -> bool {
        let block_len = block.shape.len();
        if block
            .shape
            .iter()
            .take(offset + 1)
            .zip(self.scan(block_len, offset))
            // no collision if they have no bits in common
            .all(|(b1, b2)| b1 & b2 == 0)
        {
            true
        } else {
            false
        }
    }

    fn settle_block_at(&mut self, block: &mut Block, offset: usize) {
        // check if we need to add lines on top
        let mut offset = offset;
        let block_len = block.shape.len();
        if block_len > offset {
            let new_lines = block_len - offset;
            self.lines
                .append(&mut std::iter::repeat(0).take(new_lines).collect());
            offset += new_lines;
        }
        for i in 0..block_len {
            let lines = self.lines.len();
            self.lines[lines - offset + i] |= block.shape[block_len - i - 1];
        }
    }

    fn fall_block(&mut self, mut block: Block, mut directions: impl Iterator<Item = Direction>) {
        // It pops 3 up, so we need to apply 4 times the shift
        block.shift(dbg!(directions.next().unwrap()), &self, None);
        block.shift(dbg!(directions.next().unwrap()), &self, None);
        block.shift(dbg!(directions.next().unwrap()), &self, None);
        block.shift(dbg!(directions.next().unwrap()), &self, None);

        let mut offset = 0;
        while dbg!(self.can_move_block_down(&block, offset)) {
            println!("block before \n{}", block);
            println!("board\n{}", self);
            offset += 1;
            block.shift(dbg!(directions.next().unwrap()), &self, Some(dbg!(offset)));
            println!("block after \n{}", block);
            dbg!(offset);
        }
        self.settle_block_at(&mut block, offset);
        println!("board after rest\n{}", self);
    }
}

#[derive(Debug, Clone)]
struct Block {
    shape: Vec<u8>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for line in self.shape.iter().rev() {
            let r = format!("{:07b}", line).replace("0", ".").replace("1", "#");
            writeln!(f, "{r}");
        })
    }
}

impl Block {
    fn shift(&mut self, direction: Direction, board: &Board, offset: Option<usize>) {
        match direction {
            Direction::Left => {
                // check if 7th bit is set
                if self.shape.iter().all(|x| x & (1 << 6) == 0) {
                    match offset {
                        // None means before hitting the board
                        None => {
                            self.shape = self.shape.iter().map(|b| b << 1).collect();
                        }
                        //check for collision with the board
                        Some(o) => {
                            if dbg!(self.shape.iter().take(o + 1))
                                .zip(dbg!(board.scan(self.shape.len(), o)))
                                .all(|(b1, b2)| (b1 << 1) & b2 == 0)
                            {
                                self.shape = self.shape.iter().map(|b| b << 1).collect();
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                //check if the 1st bit is set
                if self.shape.iter().all(|x| x & 1 == 0) {
                    match offset {
                        // None means before hitting the board
                        None => {
                            self.shape = self.shape.iter().map(|b| b >> 1).collect();
                        }
                        //check for collision with the board
                        Some(o) => {
                            if dbg!(self.shape.iter().take(o + 1))
                                .zip(dbg!(board.scan(self.shape.len(), o)))
                                .all(|(b1, b2)| (b1 >> 1) & b2 == 0)
                            {
                                self.shape = self.shape.iter().map(|b| b >> 1).collect();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn part1(directions: Vec<Direction>) -> usize {
    // shape 1 is [0011110] = [30]
    // shape 2 is [0001000, 0011100, 0001000] = [8, 28, 8]
    // shape 3 is [0011100, 0000100, 0000100] = [28, 4, 4]
    // shape 4 is [0010000, 0010000, 0010000, 0010000] = [16, 16, 16, 16]
    // shape 5 is [0011000, 0011000] = [24, 24]
    let b1 = Block { shape: vec![30] };
    let b2 = Block {
        shape: vec![8, 28, 8],
    };
    let b3 = Block {
        shape: vec![28, 4, 4],
    };
    let b4 = Block {
        shape: vec![16, 16, 16, 16],
    };
    let b5 = Block {
        shape: vec![24, 24],
    };
    const ITERATIONS: usize = 8;
    let mut dir = directions.into_iter().cycle();
    let blocks = vec![b1, b2, b3, b4, b5]
        .into_iter()
        .cycle()
        .take(ITERATIONS);
    let mut board = Board::new();
    for block in blocks {
        board.fall_block(block, &mut dir);
    }
    println!("{:?}", board);
    board.lines.len() - 1
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
        let mut block = Block { shape: vec![30] };
        let board = Board::new();
        block.shift(Direction::Left, &board, None);
        assert_eq!(block.shape, vec![60]);
        let mut block = Block { shape: vec![30] };
        block.shift(Direction::Right, &board, None);
        assert_eq!(block.shape, vec![15]);
    }

    #[test]
    fn test_shift_reach_side() {
        let mut block = Block { shape: vec![15] };
        let board = Board::new();
        block.shift(Direction::Right, &board, None);
        assert_eq!(block.shape, vec![15]);
        let mut block = Block { shape: vec![64] };
        block.shift(Direction::Left, &board, None);
        assert_eq!(block.shape, vec![64]);
    }

    #[test]
    fn test_shift_plus() {
        let mut plus = Block {
            shape: vec![8, 28, 8],
        };
        let board = Board::new();
        plus.shift(Direction::Left, &board, None);
        assert_eq!(plus.shape, vec![16, 56, 16]);
        let mut plus = Block {
            shape: vec![8, 28, 8],
        };
        plus.shift(Direction::Right, &board, None);
        assert_eq!(plus.shape, vec![4, 14, 4]);
    }
    #[test]
    fn test_shift_with_offset() {
        let mut plus = Block {
            shape: vec![8, 28, 8],
        };
        let mut board = Board::new();
        board.lines.append(&mut vec![11, 11, 1]);
        plus.shift(Direction::Right, &board, Some(1));
        assert_eq!(plus.shape, vec![8, 28, 8]);
        let mut plus = Block {
            shape: vec![8, 28, 8],
        };
        plus.shift(Direction::Left, &board, Some(2));
        assert_eq!(plus.shape, vec![8, 28, 8]);
    }

    #[test]
    fn test_move_down_on_board() {
        let mut board = Board::new();
        board.lines.append(&mut vec![15, 1, 1]);
        let shape = Block { shape: vec![30] };
        assert_eq!(board.can_move_block_down(&shape, 0), true);
        assert_eq!(board.can_move_block_down(&shape, 1), true);
        assert_eq!(board.can_move_block_down(&shape, 2), false);
    }

    #[test]
    fn test_scan() {
        let mut board = Board::new();
        board.lines.append(&mut vec![15, 9, 11]);
        let block_len = 2;
        assert_eq!(board.scan(block_len, 0), vec![&11]);
        assert_eq!(board.scan(block_len, 1), vec![&11, &9]);
        assert_eq!(board.scan(block_len, 2), vec![&9, &15]);
        assert_eq!(board.scan(block_len, 3), vec![&15, &127]);
    }
    #[test]
    fn test_settle_block_on_board() {
        let mut board = Board::new();
        let mut shape = Block { shape: vec![30] };
        board.settle_block_at(&mut shape, 0);
        assert_eq!(board.lines, vec![127, 30]);
        let mut shape = Block {
            shape: vec![1, 1, 1, 1],
        };
        board.settle_block_at(&mut shape, 1);
        assert_eq!(board.lines, vec![127, 31, 1, 1, 1]);
    }

    #[test]
    fn test_fall_block() {
        let mut board = Board::new();
        if let Ok(directions) = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") {
            let mut dir = directions.into_iter().cycle();
            let shape = Block { shape: vec![30] };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30]);
            let shape = Block {
                shape: vec![8, 28, 8],
            };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30, 8, 28, 8]);
            let shape = Block {
                shape: vec![4, 4, 28],
            };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30, 8, 28, 120, 16, 16]);
            let shape = Block {
                shape: vec![16, 16, 16, 16],
            };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30, 8, 28, 124, 20, 20, 4]);
            let shape = Block {
                shape: vec![24, 24],
            };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30, 8, 28, 124, 20, 20, 4, 6, 6]);
            let shape = Block { shape: vec![30] };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30, 8, 28, 124, 20, 20, 4, 6, 6, 60]);
        } else {
            panic!()
        }
    }
}
