use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;
use num::bigint::{BigUint, ToBigUint};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Mul(usize),
    Sum(usize),
    Square,
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(op: &str) -> Result<Self> {
        if op.contains('*') {
            let (_, qty) = op.split_once("* ").context("no")?;
            if qty == "old" {
                Ok(Op::Square)
            } else {
                Ok(Op::Mul(qty.parse()?))
            }
        } else if op.contains('+') {
            let (_, qty) = op.split_once("+ ").context("nope")?;
            Ok(Op::Sum(qty.parse()?))
        } else {
            Err(anyhow!("unknown op {}", &op))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Test {
    div: usize,
    yes: usize,
    no: usize,
}

impl FromStr for Test {
    type Err = Error;

    fn from_str(test: &str) -> Result<Test> {
        let mut lines = test.lines();
        let div: usize = lines
            .next()
            .context("empty")?
            .rsplit(' ')
            .next()
            .context("no space")?
            .parse()?;
        let yes: usize = lines
            .next()
            .context("empty")?
            .rsplit(' ')
            .next()
            .context("no space")?
            .parse()?;
        let no: usize = lines
            .next()
            .context("empty")?
            .rsplit(' ')
            .next()
            .context("no space")?
            .parse()?;
        Ok(Test { div, yes, no })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<BigUint>,
    op: Op,
    test: Test,
    throw_count: BigUint,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(monkey: &str) -> Result<Monkey> {
        let mut lines = monkey.lines();
        let mut chars = lines
            .next()
            .context("empty")?
            .rsplit(' ')
            .next()
            .context("no space")?
            .chars();
        chars.next_back(); // skip ':' at the end
        let id: usize = chars.as_str().parse()?;
        let (_, items) = lines
            .next()
            .context("empty")?
            .split_once(": ")
            .context("no item")?;

        let items: Vec<BigUint> = items
            .split(", ")
            .map(|i| i.trim().parse().unwrap())
            .collect();
        let op: Op = lines.next().context("empty")?.parse()?;

        let rest = format!("{}", lines.format("\n"));
        let test: Test = rest.parse()?;
        Ok(Monkey {
            id,
            items,
            op,
            test,
            throw_count: 0.to_biguint().unwrap(),
        })
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Monkey>> {
    let _input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    input.split("\n\n").map(|m| m.parse()).collect()
}

fn process(monkey: &mut Monkey) -> Vec<(usize, BigUint)> {
    let mut moves = Vec::default();
    for item in &monkey.items {
        let mut item = item.clone();
        match monkey.op {
            Op::Sum(qty) => item += qty,
            Op::Mul(qty) => item *= qty,
            Op::Square => item *= item.clone(),
        };
        item /= 3.to_biguint().unwrap();
        if &item % monkey.test.div == 0.to_biguint().unwrap() {
            moves.push((monkey.test.yes, item.clone()));
        } else {
            moves.push((monkey.test.no, item.clone()));
        }
    }
    moves
}

fn throw(monkeys: &mut Vec<Monkey>, moves: Vec<(usize, BigUint)>, monkey_id: usize) {
    for (id, item) in moves {
        monkeys[id].items.push(item)
    }
    let len: BigUint = monkeys[monkey_id].items.len().to_biguint().unwrap();
    monkeys[monkey_id].throw_count += len;
    monkeys[monkey_id].items.clear();
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let moves = process(&mut monkeys[i]);
        throw(monkeys, moves, i);
    }
}

fn compute_monkey_business(monkeys: Vec<Monkey>) -> BigUint {
    let mut throws = Vec::default();
    for monkey in monkeys {
        throws.push(monkey.throw_count);
    }
    throws.sort();
    throws[throws.len() - 1].clone() * throws[throws.len() - 2].clone()
}
pub fn part1(mut monkeys: Vec<Monkey>) -> BigUint {
    for _ in 0..20 {
        round(&mut monkeys);
    }
    compute_monkey_business(monkeys)
}

fn process_p2(monkey: &mut Monkey, constant: &BigUint) -> Vec<(usize, BigUint)> {
    let mut moves = Vec::default();
    for item in &monkey.items {
        let mut item = item.clone();
        match monkey.op {
            Op::Sum(qty) => item += qty,
            Op::Mul(qty) => item *= qty,
            Op::Square => item = item.pow(2),
        };
        item = item % constant;
        if &item % monkey.test.div == 0.to_biguint().unwrap() {
            moves.push((monkey.test.yes, item));
        } else {
            moves.push((monkey.test.no, item));
        }
    }
    moves
}
fn round_p2(monkeys: &mut Vec<Monkey>, constant: &BigUint) {
    for i in 0..monkeys.len() {
        let moves = process_p2(&mut monkeys[i], constant);
        throw(monkeys, moves, i);
    }
}

pub fn part2(mut monkeys: Vec<Monkey>) -> BigUint {
    let constant = monkeys
        .iter()
        .map(|x| x.test.div)
        .fold(1.to_biguint().unwrap(), |acc, b| acc * b);
    for _ in 0..10_000 {
        round_p2(&mut monkeys, &constant);
    }
    println!(
        "{:?}",
        monkeys
            .iter()
            .map(|m| m.throw_count.clone())
            .collect::<Vec<_>>()
    );
    compute_monkey_business(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_op_parse() {
        if let Ok(op) = "  Operation: new = old + 3".parse::<Op>() {
            assert_eq!(op, Op::Sum(3));
        }
        if let Ok(op) = "  Operation: new = old * 39".parse::<Op>() {
            assert_eq!(op, Op::Mul(39));
        }
        if let Ok(op) = "  Operation: new = old * old".parse::<Op>() {
            assert_eq!(op, Op::Square);
        }
    }

    #[test]
    fn test_test_parse() -> Result<()> {
        let test: Test = "  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            .parse()?;
        assert_eq!(
            test,
            Test {
                div: 17,
                yes: 0,
                no: 1
            }
        );
        Ok(())
    }

    #[test]
    fn test_monkey_parse() -> Result<()> {
        let m: Monkey = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            .parse()?;
        assert_eq!(
            m,
            Monkey {
                id: 0,
                items: vec![79.to_biguint().unwrap(), 98.to_biguint().unwrap()],
                op: Op::Mul(19),
                test: Test {
                    div: 23,
                    yes: 2,
                    no: 3
                },
                throw_count: 0.to_biguint().unwrap(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let mut m: Monkey = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            .parse()?;
        assert_eq!(
            process(&mut m),
            vec![
                (3, 500.to_biguint().unwrap()),
                (3, 620.to_biguint().unwrap())
            ]
        );
        Ok(())
    }
}
