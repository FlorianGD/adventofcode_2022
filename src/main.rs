use adventofcode_2022::{day01, day02, day03, day04, day05, day06, day07, day08};
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
        solution! {4, failable_parser!{ day04::parse_input }, solver!{ day04::part1 }},
        solution! {4, failable_parser!{ day04::parse_input }, solver!{ day04::part2 }},
        solution! {5, failable_parser!{ day05::parse_input }, solver!{ day05::part1 }},
        solution! {5, failable_parser!{ day05::parse_input }, solver!{ day05::part2 }},
        solution! {6, parser!{ day06::parse_input }, solver!{ day06::part1 }},
        solution! {6, parser!{ day06::parse_input }, solver!{ day06::part2 }},
        solution! {7, failable_parser!{ day07::parse_input }, solver!{ day07::part1 }},
        solution! {7, failable_parser!{ day07::parse_input }, solver!{ day07::part2 }},
        solution! {8, failable_parser!{ day08::parse_input }, solver!{ day08::part1 }},
        solution! {8, failable_parser!{ day08::parse_input }, solver!{ day08::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
