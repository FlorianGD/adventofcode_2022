use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;
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
    items: Vec<usize>,
    op: Op,
    test: Test,
    throw_count: usize,
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

        let items: Vec<usize> = items
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
            throw_count: 0,
        })
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Monkey>> {
    input.split("\n\n").map(|m| m.parse()).collect()
}

fn process(monkey: &mut Monkey) -> Vec<(usize, usize)> {
    let mut moves = Vec::default();
    for item in &monkey.items {
        let mut item = *item;
        match monkey.op {
            Op::Sum(qty) => item += qty,
            Op::Mul(qty) => item *= qty,
            Op::Square => item *= item,
        };
        item /= 3;
        if item % monkey.test.div == 0 {
            moves.push((monkey.test.yes, item));
        } else {
            moves.push((monkey.test.no, item))
        }
    }
    moves
}

fn throw(monkeys: &mut [Monkey], moves: Vec<(usize, usize)>, monkey_id: usize) {
    for (id, item) in moves {
        monkeys[id].items.push(item)
    }
    let len: usize = monkeys[monkey_id].items.len();
    monkeys[monkey_id].throw_count += len;
    monkeys[monkey_id].items.clear();
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let moves = process(&mut monkeys[i]);
        throw(monkeys, moves, i);
    }
}

fn compute_monkey_business(monkeys: Vec<Monkey>) -> usize {
    let mut throws = Vec::default();
    for monkey in monkeys {
        throws.push(monkey.throw_count);
    }
    throws.sort();
    throws[throws.len() - 1] * throws[throws.len() - 2]
}
pub fn part1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        round(&mut monkeys);
    }
    compute_monkey_business(monkeys)
}

fn process_p2(monkey: &mut Monkey, constant: &usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::default();
    for item in &monkey.items {
        let mut item = *item;
        match monkey.op {
            Op::Sum(qty) => item += qty,
            Op::Mul(qty) => item *= qty,
            Op::Square => item = item.pow(2),
        };
        item %= constant;
        if item % monkey.test.div == 0 {
            moves.push((monkey.test.yes, item));
        } else {
            moves.push((monkey.test.no, item));
        }
    }
    moves
}
fn round_p2(monkeys: &mut Vec<Monkey>, constant: &usize) {
    for i in 0..monkeys.len() {
        let moves = process_p2(&mut monkeys[i], constant);
        throw(monkeys, moves, i);
    }
}

pub fn part2(mut monkeys: Vec<Monkey>) -> usize {
    let constant = monkeys.iter().map(|x| x.test.div).product();
    for _ in 0..10_000 {
        round_p2(&mut monkeys, &constant);
    }
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
                items: vec![79, 98],
                op: Op::Mul(19),
                test: Test {
                    div: 23,
                    yes: 2,
                    no: 3
                },
                throw_count: 0,
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
        assert_eq!(process(&mut m), vec![(3, 500), (3, 620)]);
        Ok(())
    }

    #[test]
    fn test_part1() {
        let input = "Monkey 0:
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
        if let Ok(monkeys) = parse_input(input) {
            dbg!(&monkeys);
            assert_eq!(part1(monkeys), 10605);
        }
    }

    #[test]
    fn test_part2() {
        let input = "Monkey 0:
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
        if let Ok(monkeys) = parse_input(input) {
            dbg!(&monkeys);
            assert_eq!(part2(monkeys), 2713310158);
        }
    }
}
