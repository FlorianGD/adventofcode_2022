use adventofcode_2022::{day01, day02, day03};
use anyhow::Result;
use aoc_next::{aoc_main, failable_parser, parser, solution, solver, Aoc};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2022,
    solutions: &[
        solution! {1, failable_parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, failable_parser!{ day01::parse_input }, solver!{ day01::part2 }},
        solution! {2, failable_parser!{ day02::parse_input }, solver!{ day02::part1 }},
        solution! {2, failable_parser!{ day02::parse_input }, solver!{ day02::part2 }},
      solution! {3, parser!{ day03::parse_input }, solver!{ day03::part1 }},
      solution! {3, parser!{ day03::parse_input_p2 }, solver!{ day03::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
