use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn score(me: &Hand, opponent: &Hand) -> u32 {
    match (me, opponent) {
        // win is 6 points, draw 3, defeat 0
        // rock is 1 point, paper 2, scissors
        (Hand::Rock, Hand::Rock) => 4,
        (Hand::Rock, Hand::Paper) => 1,
        (Hand::Rock, Hand::Scissors) => 7,
        (Hand::Paper, Hand::Rock) => 8,
        (Hand::Paper, Hand::Paper) => 5,
        (Hand::Paper, Hand::Scissors) => 2,
        (Hand::Scissors, Hand::Rock) => 3,
        (Hand::Scissors, Hand::Paper) => 9,
        (Hand::Scissors, Hand::Scissors) => 6,
    }
}

pub fn parse_input(input: &str) -> Result<Vec<(String, String)>> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').context("wrong input")?;
            Ok((a.to_owned(), b.to_owned()))
        })
        .collect()
}

fn strat1(h: &str) -> Hand {
    match h {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => unreachable!(),
    }
}
pub fn part1(input: Vec<(String, String)>) -> u32 {
    input
        .into_iter()
        .map(|(h1, h2)| {
            let me = strat1(h2.as_str());
            let opponent = match h1.as_str() {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                _ => unreachable!("wrong input"),
            };
            score(&me, &opponent)
        })
        .sum()
}

pub fn part2(input: Vec<(String, String)>) -> u32 {
    input
        .into_iter()
        .map(|(h1, h2)| {
            let opponent = match h1.as_str() {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                _ => unreachable!("wrong input"),
            };
            let me = match (&opponent, h2.as_str()) {
                // X lose, Y draw, Z win
                (Hand::Rock, "X") => Hand::Scissors,
                (Hand::Rock, "Y") => Hand::Rock,
                (Hand::Rock, "Z") => Hand::Paper,
                (Hand::Paper, "X") => Hand::Rock,
                (Hand::Paper, "Y") => Hand::Paper,
                (Hand::Paper, "Z") => Hand::Scissors,
                (Hand::Scissors, "X") => Hand::Paper,
                (Hand::Scissors, "Y") => Hand::Scissors,
                (Hand::Scissors, "Z") => Hand::Rock,
                (_, _) => unreachable!("wrong input: {:?} {:?}", &opponent, &h1),
            };
            score(&me, &opponent)
        })
        .sum()
}
