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
            find_monkey_value(n1, d) + find_monkey_value(n2, d)
        }
        MonkeyVal::Op(Operation::Sub(n1, n2)) => {
            find_monkey_value(n1, d) - find_monkey_value(n2, d)
        }
        MonkeyVal::Op(Operation::Mul(n1, n2)) => {
            find_monkey_value(n1, d) * find_monkey_value(n2, d)
        }
        MonkeyVal::Op(Operation::Div(n1, n2)) => {
            find_monkey_value(n1, d) / find_monkey_value(n2, d)
        }
    }
}

pub fn part1(input: HashMap<String, MonkeyVal>) -> isize {
    find_monkey_value(&"root".into(), &input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MonkeyValP2 {
    Val(isize),
    Op(Operation),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
struct Eval {
    constant: f64,
    multiplier: f64,
}

fn eval(name: &String, d: &HashMap<String, MonkeyValP2>) -> Eval {
    match &d[name] {
        MonkeyValP2::Unknown => Eval {
            constant: 0.,
            multiplier: 1.,
        },
        MonkeyValP2::Val(n) => Eval {
            constant: *n as f64,
            multiplier: 0.,
        },
        MonkeyValP2::Op(op) => match op {
            Operation::Add(n1, n2) => {
                let Eval {
                    constant: left_const,
                    multiplier: left_mult,
                } = eval(n1, d);
                let Eval {
                    constant: right_const,
                    multiplier: right_mult,
                } = eval(n2, d);
                Eval {
                    constant: left_const + right_const,
                    multiplier: left_mult + right_mult,
                }
            }
            Operation::Sub(n1, n2) => {
                let Eval {
                    constant: left_const,
                    multiplier: left_mult,
                } = eval(n1, d);
                let Eval {
                    constant: right_const,
                    multiplier: right_mult,
                } = eval(n2, d);
                Eval {
                    constant: left_const - right_const,
                    multiplier: left_mult - right_mult,
                }
            }
            Operation::Mul(n1, n2) => {
                let Eval {
                    constant: left_const,
                    multiplier: left_mult,
                } = eval(n1, d);
                let Eval {
                    constant: right_const,
                    multiplier: right_mult,
                } = eval(n2, d);
                if left_mult == 0. {
                    Eval {
                        constant: left_const * right_const,
                        multiplier: left_const * right_mult,
                    }
                } else if right_mult == 0. {
                    Eval {
                        constant: left_const * right_const,
                        multiplier: left_mult * right_const,
                    }
                } else {
                    println!("eval left  {left_mult} * x +  {left_const}");
                    println!("eval right {right_mult} * x + {right_const}");
                    panic!("multiplying by x");
                }
            }
            Operation::Div(n1, n2) => {
                let Eval {
                    constant: left_const,
                    multiplier: left_mult,
                } = eval(n1, d);
                let Eval {
                    constant: right_const,
                    multiplier: right_mult,
                } = eval(n2, d);
                if right_mult != 0. {
                    panic!("dividing by x");
                } else if right_const == 0. {
                    panic!("division by 0");
                } else {
                    Eval {
                        constant: left_const / right_const,
                        multiplier: left_mult / right_const,
                    }
                }
            }
        },
    }
}

pub fn part2(input: HashMap<String, MonkeyVal>) -> isize {
    let d: HashMap<String, MonkeyValP2> = input
        .into_iter()
        .map(|(k, v)| {
            if k == "humn" {
                (k, MonkeyValP2::Unknown)
            } else {
                (
                    k,
                    match v {
                        MonkeyVal::Op(x) => MonkeyValP2::Op(x),
                        MonkeyVal::Val(n) => MonkeyValP2::Val(n),
                    },
                )
            }
        })
        .collect();
    let (
        Eval {
            constant: left_const,
            multiplier: left_mult,
        },
        Eval {
            constant: right_const,
            multiplier: right_mult,
        },
    ) = match &d["root"] {
        MonkeyValP2::Unknown => unreachable!(),
        MonkeyValP2::Val(_) => unreachable!(),
        MonkeyValP2::Op(op) => match op {
            Operation::Add(l, r) => (eval(l, &d), eval(r, &d)),
            Operation::Sub(l, r) => (eval(l, &d), eval(r, &d)),
            Operation::Mul(l, r) => (eval(l, &d), eval(r, &d)),
            Operation::Div(l, r) => (eval(l, &d), eval(r, &d)),
        },
    };

    ((right_const - left_const) / (left_mult - right_mult)) as isize
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;

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

    #[test]
    fn test_part1() -> Result<()> {
        let input = indoc! {
        "root: pppw + sjmn
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            pppw: cczh / lfqf
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32"};
        let d = parse_input(input)?;
        assert_eq!(part1(d), 152);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        let input = indoc! {
        "root: pppw + sjmn
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            pppw: cczh / lfqf
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32"};
        let d = parse_input(input)?;
        assert_eq!(part2(d), 301);
        Ok(())
    }
}
