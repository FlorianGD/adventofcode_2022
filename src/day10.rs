use anyhow::Result;

#[derive(Debug)]
pub enum Instr {
    Noop,
    Addx(isize),
}

pub fn parse_input(input: &str) -> Result<Vec<Instr>> {
    let _input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
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

fn ticks(instr: &Vec<Instr>) -> Vec<(usize, isize)> {
    let mut acc_cycle = 0usize;
    let mut acc_x = 1isize;
    instr
        .into_iter()
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

fn find_val_at_cycle(ticks: &Vec<(usize, isize)>, idx: &usize) -> isize {
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

pub fn part2(input: Vec<Instr>) -> char{
    let ticks = ticks(&input);
    println!();
    for i in 1..=240 {
        // dbg! (i , i %40);
        let x = find_val_at_cycle(&ticks, &i);
        let pixel_pos = (i as isize - 1) % 40;
        //  dbg! (x- pixel_pos);
        if (x - pixel_pos).abs() <= 1 {
            print!("█")
        } else {
            print!(" ")
        }
        if i % 40 == 0 {
            println!("")
        }
    }
    ' '
}
