use anyhow::Result;
use std::collections::HashSet;

type Point = (i32, i32, i32);

pub fn parse_input(input: &str) -> Result<Vec<Point>> {
    input
        .lines()
        .map(|l| {
            let mut v = l.split(',');
            Ok((
                v.next().unwrap().parse()?,
                v.next().unwrap().parse()?,
                v.next().unwrap().parse()?,
            ))
        })
        .collect()
}

fn neighbors(p: Point) -> HashSet<Point> {
    HashSet::from([
        (p.0 + 1, p.1, p.2),
        (p.0 - 1, p.1, p.2),
        (p.0, p.1 + 1, p.2),
        (p.0, p.1 - 1, p.2),
        (p.0, p.1, p.2 + 1),
        (p.0, p.1, p.2 - 1),
    ])
}

pub fn part1(cubes: Vec<Point>) -> usize {
    let set: HashSet<Point> = HashSet::from_iter(cubes.iter().cloned());
    cubes
        .into_iter()
        .map(|p| 6 - neighbors(p).intersection(&set).count())
        .sum()
}

fn is_point_inside(
    p: &Point,
    set: &HashSet<Point>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
) -> bool {
    // a point is inside if it not in the set and if we can reach a point in the
    // set in all the directions
    !set.contains(p)
        && (x_min..p.0).any(|x| set.contains(&(x, p.1, p.2)))
        && (p.0 + 1..=x_max).any(|x| set.contains(&(x, p.1, p.2)))
        && (y_min..p.1).any(|y| set.contains(&(p.0, y, p.2)))
        && (p.1 + 1..=y_max).any(|y| set.contains(&(p.0, y, p.2)))
        && (z_min..p.2).any(|z| set.contains(&(p.0, p.1, z)))
        && (p.2 + 1..=z_max).any(|z| set.contains(&(p.0, p.1, z)))
}

pub fn part2(mut cubes: Vec<Point>) -> usize {
    let x_min = cubes.iter().map(|p| p.0).min().unwrap();
    let x_max = cubes.iter().map(|p| p.0).max().unwrap();
    let y_min = cubes.iter().map(|p| p.1).min().unwrap();
    let y_max = cubes.iter().map(|p| p.1).max().unwrap();
    let z_min = cubes.iter().map(|p| p.2).min().unwrap();
    let z_max = cubes.iter().map(|p| p.2).max().unwrap();
    let set: HashSet<Point> = HashSet::from_iter(cubes.iter().cloned());
    // we need to discard isolated points, so the function to check if a point is inside
    // works correctly
    let set_without_isolated_points: HashSet<Point> = cubes
        .iter()
        .cloned()
        .filter(|&p| neighbors(p).intersection(&set).count() > 0)
        .collect();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                if is_point_inside(
                    &(x, y, z),
                    &set_without_isolated_points,
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    z_min,
                    z_max,
                ) {
                    cubes.push((x, y, z));
                }
            }
        }
    }
    part1(cubes)
}
