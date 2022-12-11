use anyhow::{anyhow, Context, Error, Result};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Op {
  Mul(usize), 
  Sum(usize), 
}

impl FromStr for Op {
  type Err = Error;

  fn from_str(op: &str) -> Result<Self> {
    if op.contains('*') {
      let (_, qty) = op.split_once("* ").context("no")? ;
      Ok(Op::Mul(qty.parse()?))
    } else if op.contains('+') {
      let (_, qty) = op.split_once("+ ").context("nope")? ;
      Ok(Op::Sum(qty.parse()?))
    } else {
      Err(anyhow!("unknown op {}", &op)) 
    }
   } 
}

#[derive(Debug, PartialEq)]
struct Test {
  div: usize, 
  yes: u8, 
  no : u8, 
}

impl FromStr for Test {
  type Err = Error;

  fn from_str(test: &str) -> Result<Test> {
    let mut lines = test.lines();
    let div: usize = lines.next().context("empty")?.rsplit(' ').next().context("no space")?.parse()?;
    let yes: u8 = lines.next().context("empty") ?.rsplit(' ').next().context("no space")?.parse()?;
    let no: u8 = lines.next().context("empty")?.rsplit(' ').next().context("no space")?.parse()?;
    Ok(Test { div, yes, no })
  }
}

#[derive(Debug, PartialEq) ]
pub struct Monkey {
  id: u8, 
  items: Vec<usize>, 
  op: Op, 
  test: Test
}

impl FromStr for Monkey {
  type Err = Error;
  fn from_str(monkey: &str) -> Result<Monkey> {
    let mut lines = monkey.lines();
    let mut chars = lines.next().context("empty")?.rsplit(' ').next().context("no space")?.chars();
    chars.next_back(); // skip : at the end
    let id: u8 = chars.as_str().parse()? ;
    let (_, items)= lines.next().context("empty")?.split_once(": ").context("no item")?;
    
    let items: Vec<usize>  = items.split(", ").map(|i| i.trim().parse().unwrap()).collect() ;
    let op: Op = lines.next().context("empty")?.parse()? ;
  
    let rest = format! ("{}", lines.format("\n")) ;
    dbg! (&rest);
    let test : Test = rest.parse()?;
    Ok(Monkey { id, items, op , test})
  }
}


pub fn parse_input(_input: &str) {
  //Result<Vec<Monkey>> {
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
  
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result ;
  
  #[test]
  fn test_op_parse() -> Result<()> {
    let op: Op =
      "  Operation: new = old + 3".parse()?;
    assert_eq! (op , Op::Sum(3));
    let op: Op = 
      "  Operation: new = old * 39".parse()?;
    assert_eq! (op , Op::Mul(39));
    Ok(())
  }

  #[test]
  fn test_test_parse() -> Result<() > {
    let test: Test = "  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1".parse()?;
    assert_eq!(test, Test {div: 17, yes: 0, no: 1});
    Ok(())
  }

  #[test]
  fn test_monkey_parse() -> Result<() >{
    let m: Monkey = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3".parse()?;
    assert_eq!(m, Monkey { id:0, items: vec! [79,98], op: Op::Mul(19), test : Test {div: 23, yes: 2, no: 3 } });
    Ok(())
  }
    
}