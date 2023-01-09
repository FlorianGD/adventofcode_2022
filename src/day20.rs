use std::collections::VecDeque;

use anyhow::Result;

pub fn parse_input(input: &str) -> Result<Vec<isize>> {
    // use indoc::indoc;
    // let input = indoc! {"1
    // 2
    // -3
    // 3
    // -2
    // 0
    // 4"};
    input.lines().map(|l| Ok(l.parse()?)).collect()
}

fn score(r: &VecDeque<(usize, isize)>) -> isize {
    let zero = r.iter().position(|&(_, v)| v == 0).unwrap();
    let len = r.len();
    r[(zero + 1000) % len].1 + r[(zero + 2000) % len].1 + r[(zero + 3000) % len].1
}

fn rotate<T>(d: &mut VecDeque<T>, val: isize) {
    let len = d.len();
    if val >= 0 {
        d.rotate_left(val as usize % len)
    } else {
        d.rotate_right(-val as usize % len)
    }
}

fn mix(d: &mut VecDeque<(usize, isize)>, val: (usize, isize)) {
    let idx = d.iter().position(|&v| v == val).unwrap();
    d.rotate_left(idx);
    assert_eq!(d.pop_front(), Some(val));
    rotate(d, val.1);
    d.push_back(val);
    rotate(d, -val.1);
}

pub fn part1(input: Vec<isize>) -> isize {
    let mut queue = VecDeque::from_iter(input.into_iter().enumerate());
    let mut list = queue.clone();
    while let Some(val) = queue.pop_front() {
        mix(&mut list, val);
    }
    score(&list)
}
