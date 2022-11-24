use adventofcode_2022::day01;
use anyhow::Result;
use aoc_next::{aoc_main, failable_parser, solution, solver, Aoc};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2021,
    solutions: &[
        solution! {1, failable_parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, failable_parser!{ day01::parse_input }, solver!{ day01::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
