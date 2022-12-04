use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from_tuple(t: (u32, u32)) -> Self {
        Range {
            start: t.0,
            end: t.1,
        }
    }
    fn from_str(s: &str) -> Result<Self> {
        let (t1, t2) = s.split_once("-").context("no dash")?;
        Ok(Self::from_tuple((t1.parse()?, t2.parse()?)))
    }
    fn included(&self, other: &Range) -> bool {
        self.start >= other.start && self.end <= other.end
    }
    fn overlap(&self, other: &Range) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}

type Pair = (Range, Range);

pub fn parse_input(input: &str) -> Result<Vec<Pair>> {
    // let input = "2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8";
    input
        .lines()
        .map(|l| {
            let (t1, t2) = l.split_once(",").context("no separator")?;
            Ok((Range::from_str(t1)?, Range::from_str(t2)?))
        })
        .collect()
}

pub fn part1(input: Vec<Pair>) -> usize {
    input
        .iter()
        .filter(|(r1, r2)| r1.included(r2) || r2.included(r1))
        .count()
}

pub fn part2(input: Vec<Pair>) -> usize {
    input.iter().filter(|(r1, r2)| r1.overlap(r2)).count()
}
