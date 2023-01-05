use anyhow::Result;

#[derive(Debug)]
pub enum Instr {
    Noop,
    Addx(isize),
}

pub fn parse_input(input: &str) -> Result<Vec<Instr>> {
    input
        .lines()
        .map(|l| {
            Ok(match &l[..4] {
                "noop" => Instr::Noop,
                "addx" => Instr::Addx(l[5..].parse()?),
                _ => unreachable!("unknown op"),
            })
        })
        .collect()
}

fn ticks(instr: &[Instr]) -> Vec<(usize, isize)> {
    let mut acc_cycle = 0usize;
    let mut acc_x = 1isize;
    instr
        .iter()
        .map(|instr| {
            match instr {
                Instr::Noop => {
                    acc_cycle += 1;
                }
                Instr::Addx(x) => {
                    acc_cycle += 2;
                    acc_x += x;
                }
            }
            (acc_cycle, acc_x)
        })
        .collect()
}

fn find_val_at_cycle(ticks: &[(usize, isize)], idx: &usize) -> isize {
    let idx = ticks.iter().take_while(|(cycle, _)| cycle < idx).count();
    if idx == 0 {
        return 1;
    }
    ticks[idx - 1].1
}

pub fn part1(input: Vec<Instr>) -> isize {
    let ticks = ticks(&input);
    [20usize, 60, 100, 140, 180, 220]
        .into_iter()
        .fold(0isize, |acc, idx| {
            acc + (idx as isize) * find_val_at_cycle(&ticks, &idx)
        })
}

pub fn part2(input: Vec<Instr>) -> char {
    let ticks = ticks(&input);
    println!();
    for i in 1..=240 {
        let x = find_val_at_cycle(&ticks, &i);
        let pixel_pos = (i as isize - 1) % 40;
        if (x - pixel_pos).abs() <= 1 {
            print!("█")
        } else {
            print!(" ")
        }
        if i % 40 == 0 {
            println!()
        }
    }
    ' '
}
