use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::{sequence::delimited, IResult};

type Pair = (Packet, Packet);
use itertools::EitherOrBoth::{Both, Left, Right};
#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Num(u32),
    Values(Vec<Packet>),
}

fn digit(s: &str) -> IResult<&str, Packet> {
    map_res(digit1, |i: &str| match i.parse() {
        Ok(n) => Ok(Packet::Num(n)),
        Err(e) => Err(e),
    })(s)
}

fn numbers(s: &str) -> IResult<&str, Packet> {
    match separated_list0(char(','), alt((digit, packet)))(s) {
        Ok((i, n)) => Ok((i, Packet::Values(n))),
        Err(e) => Err(e),
    }
}

fn packet(s: &str) -> IResult<&str, Packet> {
    delimited(char('['), alt((numbers, packet)), char(']'))(s)
}

pub fn parse_input(input: &str) -> Result<Vec<Pair>> {
    input
        .split("\n\n")
        .map(|lines| {
            if let Some((Ok(left), Ok(right))) = lines
                .lines()
                .map(|l| match packet(l) {
                    Ok((_i, j)) => Ok(j),
                    Err(e) => Err(e),
                })
                .collect_tuple()
            {
                Ok((left, right))
            } else {
                Err(anyhow!("Something wrong happened in parsing {:?}", lines))
            }
        })
        .collect()
}

fn compare_pair(left: Packet, right: Packet) -> Option<bool> {
    // println!("left {:?}, right {:?}", &left, &right);
    match (left, right) {
        (Packet::Num(i), Packet::Num(j)) if i < j => Some(true),
        (Packet::Num(i), Packet::Num(j)) if i > j => Some(false),
        (Packet::Num(i), Packet::Num(j)) if i == j => None,
        (Packet::Num(_), Packet::Num(_)) => unreachable!("make the compiler happy"),
        (Packet::Num(i), Packet::Values(r)) => {
            compare_pair(Packet::Values(vec![Packet::Num(i)]), Packet::Values(r))
        }
        (Packet::Values(l), Packet::Num(j)) => {
            compare_pair(Packet::Values(l), Packet::Values(vec![Packet::Num(j)]))
        }

        (Packet::Values(l), Packet::Values(r)) => {
            for a in l.into_iter().zip_longest(r) {
                match a {
                    Both(l_, r_) => match compare_pair(l_, r_) {
                        Some(p) => return Some(p),
                        None => continue,
                    },
                    Left(_) => return Some(false),
                    Right(_) => return Some(true),
                }
            }
            None
        }
    }
}

pub fn part1(input: Vec<Pair>) -> usize {
    input
        .into_iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if let Some(p) = compare_pair(left, right) {
                if p {
                    Some(i + 1)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

pub fn parse_input_p2(input: &str) -> Result<Vec<Packet>> {
    let input_p2 = format!("{input}\n[[2]]\n[[6]]");
    input_p2
        .lines()
        .filter(|&l| !l.trim().is_empty())
        .map(|line| match packet(line) {
            Ok((_i, p)) => Ok(p),
            Err(_) => Err(anyhow!("Something wrong happened in parsing {:?}", line)),
        })
        .collect()
}

pub fn part2(mut input: Vec<Packet>) -> usize {
    input.sort_by(|l, r| match compare_pair(l.clone(), r.clone()) {
        None => Ordering::Equal,
        Some(p) => {
            if p {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    });
    let two = Packet::Values(vec![Packet::Values(vec![Packet::Num(2)])]);
    let six = Packet::Values(vec![Packet::Values(vec![Packet::Num(6)])]);
    input
        .into_iter()
        .enumerate()
        .filter(|(_, p)| p == &two || p == &six)
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            packet("[[[]]]"),
            Ok((
                "",
                Packet::Values(vec![Packet::Values(vec![Packet::Values(vec![])])])
            ))
        );
        assert_eq!(
            packet("[[8,7,6]]"),
            Ok((
                "",
                Packet::Values(vec![Packet::Values(vec![
                    Packet::Num(8),
                    Packet::Num(7),
                    Packet::Num(6),
                ])])
            ))
        );
        assert_eq!(packet("[]"), Ok(("", Packet::Values(vec![]))));
        assert_eq!(
            packet("[1,2,3]"),
            Ok((
                "",
                Packet::Values(vec![Packet::Num(1), Packet::Num(2), Packet::Num(3)])
            ))
        );
        assert_eq!(
            packet("[1,[2,[3]]]"),
            Ok((
                "",
                Packet::Values(vec![
                    Packet::Num(1),
                    Packet::Values(vec![Packet::Num(2), Packet::Values(vec![Packet::Num(3)])])
                ])
            ))
        );
        // assert_eq!(parse_input(b"[]"), Ok("");
        // assert_eq!(parse_input(b"1"), Ok(1));
        // assert_eq!(parse_input(b"[1]"), Ok(vec![1]));
    }

    #[test]
    fn test_part1() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        if let Ok(i) = parse_input(input) {
            assert_eq!(part1(i), 13)
        }
    }

    #[test]
    fn test_part2() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        if let Ok(i) = parse_input_p2(input) {
            assert_eq!(part2(i), 140)
        }
    }
}
