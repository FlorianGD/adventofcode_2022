use anyhow::{anyhow, Error, Result};
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::{preceded, tuple};
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

type Coord = (isize, isize);

fn maybe_neg_num(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((i, number))
}

fn sensor_x(s: &str) -> IResult<&str, isize> {
    preceded(tag("Sensor at x="), maybe_neg_num)(s)
}

fn y(s: &str) -> IResult<&str, isize> {
    preceded(tag(", y="), maybe_neg_num)(s)
}
fn beacon_x(s: &str) -> IResult<&str, isize> {
    preceded(tag(": closest beacon is at x="), maybe_neg_num)(s)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    sensor: Coord,
    beacon: Coord,
}

fn parse_line(s: &str) -> IResult<&str, Report> {
    map_res(
        tuple((sensor_x, y, beacon_x, y)),
        |(sens_x, sens_y, beac_x, beac_y)| {
            let r = Report {
                sensor: (sens_x, sens_y),
                beacon: (beac_x, beac_y),
            };
            Ok::<_, Error>(r)
        },
    )(s)
}

fn l1_norm((x1, y1): &Coord, (x2, y2): &Coord) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn parse_input(input: &str) -> Result<Vec<Report>> {
    //     let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    // Sensor at x=9, y=16: closest beacon is at x=10, y=16
    // Sensor at x=13, y=2: closest beacon is at x=15, y=3
    // Sensor at x=12, y=14: closest beacon is at x=10, y=16
    // Sensor at x=10, y=20: closest beacon is at x=10, y=16
    // Sensor at x=14, y=17: closest beacon is at x=10, y=16
    // Sensor at x=8, y=7: closest beacon is at x=2, y=10
    // Sensor at x=2, y=0: closest beacon is at x=2, y=10
    // Sensor at x=0, y=11: closest beacon is at x=2, y=10
    // Sensor at x=20, y=14: closest beacon is at x=25, y=17
    // Sensor at x=17, y=20: closest beacon is at x=21, y=22
    // Sensor at x=16, y=7: closest beacon is at x=15, y=3
    // Sensor at x=14, y=3: closest beacon is at x=15, y=3
    // Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    input
        .lines()
        .map(|l| match parse_line(l) {
            Ok((_, p)) => Ok(p),
            Err(_) => Err(anyhow!("Error in parsing line {}", l)),
        })
        .collect()
}

fn find_blocked_at_line(report: &Report, line: isize) -> Option<(isize, isize)> {
    let dist_to_beacon = l1_norm(&report.sensor, &report.beacon);
    let dist_to_line = l1_norm(&report.sensor, &(report.sensor.0, line));
    let diff = dist_to_beacon - dist_to_line;
    if diff < 0 {
        return None;
    }
    // The line is within the beacon distance
    Some((report.sensor.0 - diff, report.sensor.0 + diff))
}

fn merge_overlapping_intervals(arr: &mut Vec<Coord>) -> Vec<Coord> {
    arr.sort();

    let mut result: Vec<Coord> = Vec::new();
    result.push(arr[0].clone());

    for i in 1..arr.len() {
        let current: Coord = arr[i].clone();
        let j: usize = result.len() - 1;

        if current.0 >= result[j].0 && current.0 <= result[j].1 {
            let current_max = (current.1).max(result[j].1);
            result[j] = (result[j].0, current_max);
        } else {
            result.push(current);
        }
    }
    result
}

pub fn part1(reports: Vec<Report>) -> isize {
    // const LINE: isize = 10;
    const LINE: isize = 2000000;

    let mut ranges = reports
        .iter()
        .filter_map(|r| find_blocked_at_line(r, LINE))
        .collect::<Vec<_>>();

    merge_overlapping_intervals(&mut ranges)
        .iter()
        .map(|(x, y)| y - x)
        .sum()
}

pub fn part2(reports: Vec<Report>) -> isize {
    for y in 0..=4000000 {
        let mut ranges = reports
            .iter()
            .filter_map(|r| find_blocked_at_line(r, y))
            .collect::<Vec<_>>();
        let merged = merge_overlapping_intervals(&mut ranges);
        if merged.len() > 1 {
            if merged[1].0 - merged[0].1 > 1 {
                return 4000000 * (merged[0].1 + 1) + y;
            }
            println!("{:?}, {:?}", merged, y);
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neg_number() {
        assert_eq!(maybe_neg_num("1002"), Ok(("", 1002)));
        assert_eq!(maybe_neg_num("-1002"), Ok(("", -1002)));
    }

    #[test]
    fn test_sensor_x() {
        assert_eq!(sensor_x("Sensor at x=1002"), Ok(("", 1002)));
        assert_eq!(sensor_x("Sensor at x=-1002"), Ok(("", -1002)));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Ok((
                "",
                Report {
                    sensor: (2, 18),
                    beacon: (-2, 15)
                }
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        if let Ok(s) = parse_input(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        ) {
            assert_eq!(
                s,
                vec![
                    Report {
                        sensor: (2, 18),
                        beacon: (-2, 15)
                    },
                    Report {
                        sensor: (9, 16),
                        beacon: (10, 16)
                    }
                ]
            )
        }
    }
}
