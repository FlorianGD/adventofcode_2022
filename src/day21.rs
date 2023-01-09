use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::error::Error;
use nom::sequence::{separated_pair, terminated};
use nom::{Err, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonkeyVal {
    Val(isize),
    Op(Operation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn monkey_name(s: &str) -> IResult<&str, &str> {
    alpha1(s)
}

fn name(s: &str) -> IResult<&str, &str> {
    terminated(monkey_name, tag(": "))(s)
}

fn number(s: &str) -> IResult<&str, MonkeyVal> {
    match digit1(s) {
        Ok((i, r)) => match r.parse() {
            Ok(n) => Ok((i, MonkeyVal::Val(n))),
            Err(_) => Err(Err::Error(Error::new(
                "error parsing",
                nom::error::ErrorKind::Digit,
            ))),
        },
        Err(e) => Err(e),
    }
}

fn addition(s: &str) -> IResult<&str, MonkeyVal> {
    match separated_pair(monkey_name, tag(" + "), monkey_name)(s) {
        Ok((i, (n1, n2))) => Ok((i, MonkeyVal::Op(Operation::Add(n1.into(), n2.into())))),
        Err(e) => Err(e),
    }
}
fn subtraction(s: &str) -> IResult<&str, MonkeyVal> {
    match separated_pair(monkey_name, tag(" - "), monkey_name)(s) {
        Ok((i, (n1, n2))) => Ok((i, MonkeyVal::Op(Operation::Sub(n1.into(), n2.into())))),
        Err(e) => Err(e),
    }
}
fn multiplication(s: &str) -> IResult<&str, MonkeyVal> {
    match separated_pair(monkey_name, tag(" * "), monkey_name)(s) {
        Ok((i, (n1, n2))) => Ok((i, MonkeyVal::Op(Operation::Mul(n1.into(), n2.into())))),
        Err(e) => Err(e),
    }
}
fn division(s: &str) -> IResult<&str, MonkeyVal> {
    match separated_pair(monkey_name, tag(" / "), monkey_name)(s) {
        Ok((i, (n1, n2))) => Ok((i, MonkeyVal::Op(Operation::Div(n1.into(), n2.into())))),
        Err(e) => Err(e),
    }
}

fn operation(s: &str) -> IResult<&str, MonkeyVal> {
    alt((number, addition, subtraction, multiplication, division))(s)
}

fn line(s: &str) -> IResult<&str, (String, MonkeyVal)> {
    match name(s) {
        Ok((i, n)) => match alt((number, operation))(i) {
            Ok((i, v)) => Ok((i, (n.into(), v))),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
pub fn parse_input(input: &str) -> Result<HashMap<String, MonkeyVal>> {
    // use indoc::indoc;
    // let input = indoc! { "root: pppw + sjmn
    // dbpl: 5
    // cczh: sllz + lgvd
    // zczc: 2
    // ptdq: humn - dvpt
    // dvpt: 3
    // lfqf: 4
    // humn: 5
    // ljgn: 2
    // sjmn: drzm * dbpl
    // sllz: 4
    // pppw: cczh / lfqf
    // lgvd: ljgn * ptdq
    // drzm: hmdt - zczc
    // hmdt: 32"};
    input
        .lines()
        .map(|l| match line(l) {
            Ok((_, m)) => Ok(m),
            Err(e) => {
                println!("{:?}", e);
                Err(anyhow!("parsing error"))
            }
        })
        .collect()
}

fn find_monkey_value(name: &String, d: &HashMap<String, MonkeyVal>) -> isize {
    match &d[name] {
        MonkeyVal::Val(n) => *n,
        MonkeyVal::Op(Operation::Add(n1, n2)) => {
            find_monkey_value(&n1, &d) + find_monkey_value(&n2, &d)
        }
        MonkeyVal::Op(Operation::Sub(n1, n2)) => {
            find_monkey_value(&n1, &d) - find_monkey_value(&n2, &d)
        }
        MonkeyVal::Op(Operation::Mul(n1, n2)) => {
            find_monkey_value(&n1, &d) * find_monkey_value(&n2, &d)
        }
        MonkeyVal::Op(Operation::Div(n1, n2)) => {
            find_monkey_value(&n1, &d) / find_monkey_value(&n2, &d)
        }
    }
}

pub fn part1(input: HashMap<String, MonkeyVal>) -> isize {
    find_monkey_value(&"root".into(), &input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(name("aaaa: "), Ok(("", "aaaa")))
    }

    #[test]
    fn test_operation() {
        assert_eq!(
            operation("aaaa + bbbb"),
            Ok((
                "",
                MonkeyVal::Op(Operation::Add("aaaa".into(), "bbbb".into()))
            ))
        );
        assert_eq!(
            operation("aaaa - bbbb"),
            Ok((
                "",
                MonkeyVal::Op(Operation::Sub("aaaa".into(), "bbbb".into()))
            ))
        );
        assert_eq!(
            operation("aaaa * bbbb"),
            Ok((
                "",
                MonkeyVal::Op(Operation::Mul("aaaa".into(), "bbbb".into()))
            ))
        );
        assert_eq!(
            operation("aaaa / bbbb"),
            Ok((
                "",
                MonkeyVal::Op(Operation::Div("aaaa".into(), "bbbb".into()))
            ))
        );
    }

    #[test]
    fn test_line_number() {
        assert_eq!(
            line("aaaa: 10"),
            Ok(("", ("aaaa".into(), MonkeyVal::Val(10))))
        )
    }
    #[test]
    fn test_line_operation() {
        assert_eq!(
            line("aaaa: bbbb + cccc"),
            Ok((
                "",
                (
                    "aaaa".into(),
                    MonkeyVal::Op(Operation::Add("bbbb".into(), "cccc".into()))
                )
            ))
        )
    }
}
