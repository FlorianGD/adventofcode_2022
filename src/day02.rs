use anyhow::Result ;

#[derive( Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn score(me: Hand, opponent: Hand) -> u32 {
  match (me, opponent) {
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

pub fn parse_input(_input: & str) -> Result<Vec<(&str, & str)> > {
  let input = "A Y
B X
C Z";
  input.lines().map(|line| {
    let (a, b) = line.split_once(" ").unwrap() ;
    Ok((a, b)) 
  }).collect()
}


fn strat1(h: & str) -> Hand {
  match h {
    "X" => Hand::Rock, 
    "Y" => Hand::Paper, 
    "Z" => Hand::Scissors, 
    _ => unreachable!()
  }
}
pub fn part1(input: Vec<(&str, & str)>) -> u32 {
  input.into_iter().map(|(h1, h2)| {
    let me = strat1(h2);
    let opponent = match h1 {
      "A" => Hand::Rock, 
      "B" => Hand::Paper, 
      "C" => Hand::Scissors, 
      _ => unreachable!()
    };
    score(me, opponent )
  }).sum()
}