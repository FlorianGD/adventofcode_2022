use anyhow::{anyhow, Result};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
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
    // let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    input.trim().chars().map(Direction::from_char).collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Board {
    lines: Vec<u8>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = String::new();
        for line in self.lines.iter().rev() {
            r = format!("{}{:07b}\n", r, line)
                .replace('0', ".")
                .replace('1', "#");
        }
        write!(f, "{r}")
    }
}
impl Board {
    fn new() -> Self {
        Board {
            lines: vec![0b1111111],
        }
    }

    fn scan(&self, block_len: usize, offset: usize) -> Vec<&u8> {
        if block_len > offset {
            self.lines.iter().rev().take(offset).rev().collect()
        } else {
            self.lines
                .iter()
                .rev()
                .skip(offset - block_len)
                .take(block_len)
                .rev()
                .collect()
        }
    }

    fn can_move_block_down(&self, block: &Block, offset: usize) -> bool {
        let block_len = block.shape.len();
        block
            .shape
            .iter()
            .take(offset + 1)
            .zip(self.scan(block_len, offset + 1))
            // no collision if they have no bits in common
            .all(|(b1, b2)| b1 & b2 == 0)
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
            self.lines[lines - offset + i] |= block.shape[i];
        }
    }

    fn fall_block(
        &mut self,
        mut block: Block,
        mut directions: impl Iterator<Item = Direction>,
    ) -> usize {
        // It pops 3 up, so we need to apply 4 times the shift
        block.shift(directions.next().unwrap(), self, None);
        block.shift(directions.next().unwrap(), self, None);
        block.shift(directions.next().unwrap(), self, None);
        block.shift(directions.next().unwrap(), self, None);

        let mut offset = 0;
        while self.can_move_block_down(&block, offset) {
            offset += 1;
            block.shift(directions.next().unwrap(), self, Some(offset));
        }
        self.settle_block_at(&mut block, offset);
        // number of directions consumed
        4 + offset
    }

    /// Return the floor for each index, i.e. the first where it is 1.
    /// Safety: the first line is 127, so we can unwrap safely
    fn floor(&self) -> Vec<u8> {
        (0..7)
            .map(|i| {
                self.lines
                    .iter()
                    .rev()
                    .position(|&x| x & (1 << i) != 0)
                    .unwrap() as u8
            })
            .collect()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Block {
    shape: Vec<u8>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = String::new();
        for line in self.shape.iter().rev() {
            r = format!("{}{:07b}", r, line)
                .replace('0', ".")
                .replace('1', "#");
        }
        write!(f, "{r}")
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
                            if self
                                .shape
                                .iter()
                                .take(o)
                                .zip(board.scan(self.shape.len(), o))
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
                            if self
                                .shape
                                .iter()
                                .take(o)
                                .zip(board.scan(self.shape.len(), o))
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

fn play_part1(directions: Vec<Direction>, iterations: usize) -> usize {
    // shape 1 is "—" [0011110] = [30]
    // shape 2 is "+" [0001000, 0011100, 0001000] = [8, 28, 8]
    // shape 3 is "L" reversed [0011100, 0000100, 0000100] = [28, 4, 4] // bottom first
    // shape 4 is "|" [0010000, 0010000, 0010000, 0010000] = [16, 16, 16, 16]
    // shape 5 is "■" [0011000, 0011000] = [24, 24]
    let b1 = Block {
        shape: vec![0b0011110],
    };
    let b2 = Block {
        shape: vec![0b0001000, 0b0011100, 0b0001000],
    };
    let b3 = Block {
        shape: vec![0b0011100, 0b0000100, 0b0000100],
    };
    let b4 = Block {
        shape: vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
    };
    let b5 = Block {
        shape: vec![0b0011000, 0b0011000],
    };
    let mut dir = directions.into_iter().cycle();
    let blocks = vec![b1, b2, b3, b4, b5]
        .into_iter()
        .cycle()
        .take(iterations);
    let mut board = Board::new();
    for block in blocks {
        board.fall_block(block, &mut dir);
    }
    board.lines.len() - 1
}

pub fn part1(directions: Vec<Direction>) -> usize {
    play_part1(directions, 2022)
}

fn play_part2(directions: Vec<Direction>, iterations: usize) -> usize {
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
    let dir_len = directions.len();
    let block_len = 5;
    let mut dir = directions.into_iter().cycle();
    let blocks = vec![b1, b2, b3, b4, b5].into_iter().cycle();
    // cache is (block, direction, floor, blocks added in step)
    let mut cache: Vec<(usize, usize, Vec<u8>, usize)> = Vec::new();
    let mut board = Board::new();
    let mut prev_count = 0;
    let mut current_dir = 0;
    for (i, block) in blocks.enumerate() {
        current_dir += board.fall_block(block, &mut dir);
        let floor = board.floor();
        match cache.iter().position(|(block, direction, f, _)| {
            *block == (i % block_len) && *direction == (current_dir % dir_len) && *f == floor
        }) {
            None => cache.push((
                i % block_len,
                current_dir % dir_len,
                floor.clone(),
                board.lines.len() - 1 - prev_count,
            )),
            Some(p) => {
                let blocks_added_in_cycle: usize = cache.iter().skip(p).map(|x| x.3).sum();
                let cycle_size = i - p;
                let div = (iterations - p) / cycle_size;
                let remainder = (iterations - p) % cycle_size;
                let remaining_blocks: usize = cache.iter().take(p + remainder).map(|x| x.3).sum();
                return blocks_added_in_cycle * div + remaining_blocks;
            }
        }
        prev_count = board.lines.len() - 1;
    }
    unreachable!()
}

pub fn part2(directions: Vec<Direction>) -> usize {
    play_part2(directions, 1000000000000)
}
//1524110593571 too low for part 2

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
            shape: vec![16, 56, 16],
        };
        // plus is
        // ..#....
        // .###...
        // ..#....
        let mut board = Board::new();
        board.lines.append(&mut vec![11, 11, 1]);
        println!("{}", plus);
        // board is
        // ......#
        // ...#.##
        // ...#.##
        // #######
        // with offset 1, we can move right
        plus.shift(Direction::Right, &board, Some(1));
        assert_eq!(plus.shape, vec![8, 28, 8]);
        let mut plus = Block {
            shape: vec![16, 56, 16],
        };
        // with offset 2, we cannot move right
        plus.shift(Direction::Right, &board, Some(2));
        assert_eq!(plus.shape, vec![16, 56, 16]);
        // with offset 2, we can move left
        let mut plus = Block {
            shape: vec![16, 56, 16],
        };
        plus.shift(Direction::Left, &board, Some(2));
        assert_eq!(plus.shape, vec![32, 112, 32]);
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
        // println!("{:?}", board.lines);
        let block_len = 2;
        assert_eq!(board.scan(block_len, 0), Vec::<&u8>::default());
        assert_eq!(board.scan(block_len, 1), vec![&11]);
        assert_eq!(board.scan(block_len, 2), vec![&9, &11]);
        assert_eq!(board.scan(block_len, 3), vec![&15, &9]);
        assert_eq!(board.scan(block_len, 4), vec![&127, &15]);
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
            assert_eq!(board.lines, vec![0b1111111, 30]);
            let shape = Block {
                shape: vec![8, 28, 8],
            };
            board.fall_block(shape, &mut dir);
            assert_eq!(board.lines, vec![127, 30, 8, 28, 8]);
            let shape = Block {
                shape: vec![28, 4, 4],
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

    #[test]
    fn test_part1() {
        if let Ok(directions) = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") {
            assert_eq!(part1(directions), 3068)
        } else {
            panic!()
        }
    }
    #[test]
    fn test_play() {
        if let Ok(directions) = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") {
            assert_eq!(play_part1(directions.clone(), 2022), 3068);
            assert_eq!(play_part2(directions, 2022), 3068);
        } else {
            panic!()
        }
    }
    #[test]
    fn test_play_part2() {
        if let Ok(directions) = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") {
            assert_eq!(play_part2(directions.clone(), 2022), 3068);
            assert_eq!(play_part2(directions, 1000000000000), 1514285714288);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_part2() {
        if let Ok(directions) = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") {
            assert_eq!(part2(directions), 1514285714288)
        } else {
            panic!()
        }
    }
}
