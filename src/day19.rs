use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::{equal, sorted};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// type Cache = Lazy<Mutex<HashMap<(Vec<usize>, Vec<usize>, usize), usize>>>;
// static CACHE: Cache = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Recipes = HashMap<Resource, HashMap<Resource, usize>>;
type Robots = HashMap<Resource, usize>;
type Resources = HashMap<Resource, usize>;

#[derive(Debug, Clone)]
pub struct Blueprint {
    id: usize,
    recipes: Recipes,
}

fn number(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(s)
}

fn parse_line(s: &str) -> IResult<&str, (usize, usize, usize, (usize, usize), (usize, usize))> {
    tuple((
        delimited(tag("Blueprint "), number, tag(": ")),
        delimited(tag("Each ore robot costs "), number, tag(" ore. ")),
        delimited(tag("Each clay robot costs "), number, tag(" ore. ")),
        tuple((
            delimited(tag("Each obsidian robot costs "), number, tag(" ore ")),
            delimited(tag("and "), number, tag(" clay. ")),
        )),
        tuple((
            delimited(tag("Each geode robot costs "), number, tag(" ore ")),
            delimited(tag("and "), number, tag(" obsidian.")),
        )),
    ))(s)
}

pub fn parse_input(_input: &str) -> Result<Vec<Blueprint>> {
    use indoc::indoc;
    let input = indoc! {"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."};
    //Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."};
    input.lines().map(|l| l.parse()).collect()
}

impl FromStr for Blueprint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Ok((
            _,
            (
                id,
                ore_robot_cost_in_ore,
                clay_robot_cost_in_ore,
                (obs_robot_cost_in_ore, obs_robot_cost_in_clay),
                (geode_robot_cost_in_ore, geode_robot_cost_in_obsidian),
            ),
        )) = parse_line(s)
        {
            Ok(Blueprint {
                id,
                recipes: [
                    (
                        Resource::Ore,
                        vec![(Resource::Ore, ore_robot_cost_in_ore)]
                            .into_iter()
                            .collect(),
                    ),
                    (
                        Resource::Clay,
                        vec![(Resource::Ore, clay_robot_cost_in_ore)]
                            .into_iter()
                            .collect(),
                    ),
                    (
                        Resource::Obsidian,
                        vec![
                            (Resource::Ore, obs_robot_cost_in_ore),
                            (Resource::Clay, obs_robot_cost_in_clay),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    (
                        Resource::Geode,
                        vec![
                            (Resource::Ore, geode_robot_cost_in_ore),
                            (Resource::Obsidian, geode_robot_cost_in_obsidian),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ]
                .into_iter()
                .collect(),
            })
        } else {
            Err(anyhow!("Could not parse {s} as a Blueprint"))
        }
    }
}

impl Blueprint {
    // /// Consume the amount of resources to produce the target robot.
    // fn produce(&self, resources: &mut Resources, target: &Resource) {
    //     if let Some(needed_resources) = self.recipes.get(target) {
    //         for (r, q) in needed_resources {
    //             resources.entry(*r).and_modify(|e| *e -= q);
    //         }
    //     }
    // }

    fn max_needed_robot_per_resource(&self) -> HashMap<Resource, usize> {
        [Resource::Ore, Resource::Clay, Resource::Obsidian]
            .into_iter()
            .map(|r| {
                (
                    r,
                    *self
                        .recipes
                        .iter()
                        .map(|(_, v)| v.get(&r).unwrap_or(&0))
                        .max()
                        .unwrap_or(&usize::MAX),
                )
            })
            .collect()
    }
    /// Let's see how long we need to wait until we can build one kind of robot and jump
    /// directly to it.
    fn next_possible_robots(
        &self,
        resources: &Resources,
        robots: &Robots,
        time_left: usize,
    ) -> Vec<(Resources, Robots, usize)> {
        println!("new entry in function with time left {time_left} and:\nressources\t{:?}\nrobots\t\t{:?}\n", &resources, &robots);
        let mut possible = vec![];
        for (robot_to_produce, needed_resources) in &self.recipes {
            // if we already have built the maximum number of robot that can produce one
            // resource
            dbg!(robot_to_produce);
            dbg!(&needed_resources);
            if robots.get(&robot_to_produce).unwrap_or(&0)
                == self
                    .max_needed_robot_per_resource()
                    .get(&robot_to_produce)
                    .unwrap_or(&usize::MAX)
            {
                println!("don't need to produce {robot_to_produce:?}");
                continue;
            }
            // can I create this resource, and if yes, when?
            let maybe_time: Vec<Option<usize>> = needed_resources
                .iter()
                .map(|(resource, quantity)| {
                    let number_we_have = resources.get(&resource).unwrap_or(&0);
                    if number_we_have > quantity {
                        Some(0)
                    } else {
                        let number_we_need = dbg!(quantity - number_we_have);
                        // with constant resources, as we compute the time til we can build
                        // the next robot.
                        if let Some(number_by_minute) = robots.get(&resource) {
                            if dbg!(number_by_minute) == &0 {
                                None
                            } else {
                                Some(dbg!((number_we_need / number_by_minute).max(1)))
                            }
                        } else {
                            None
                        }
                    }
                })
                .collect();

            if dbg!(&maybe_time).iter().any(|v| v.is_none()) {
                // we cannot build the robot with those resources
                println!("Cannot build {robot_to_produce:?}");
                continue;
            }
            // let's compute the resources we are going to have when we can build
            // the robot
            let time_when_ready = maybe_time.iter().max().unwrap().unwrap();
            // -1 because if the robot is ready on the last minute, it cannot
            // produce any resource
            if dbg!(time_when_ready) > time_left - 2 {
                println!("Not enough time to build {robot_to_produce:?}");
                continue;
            }
            // we can build a robot, compute the new resources we'll get
            // We will have the robot ready in time_when_ready minutes
            // In the meantime, we will produce time_when_ready + 1 * the number of
            // robot.
            // We will consume the resources needed to build the new robot
            // We will have one more resource of the new robot, and we will consume
            // time_when_ready+1 minutes because we can only build one robot per minute.
            println!("possible to build {robot_to_produce:?}");
            let mut new_resources = dbg!(resources.clone());
            let mut new_robots = dbg!(robots.clone());

            for r in vec![
                Resource::Ore,
                Resource::Clay,
                Resource::Obsidian,
                Resource::Geode,
            ] {
                dbg!(r);
                let q1 = dbg!(new_resources.entry(r).or_insert(0));
                *q1 += dbg!((time_when_ready + 1) * new_robots.get(&r).unwrap_or(&0));
                *q1 -= dbg!(needed_resources.get(&r).unwrap_or(&0));
            }
            new_robots
                .entry(*robot_to_produce)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            new_resources
                .entry(*robot_to_produce)
                .and_modify(|e| *e += 1);
            possible.push((new_resources, new_robots, time_left - time_when_ready - 1));
        }
        dbg!(possible)
    }

    // /// Allowed robots by the recipe and the given resources.
    // fn possible_robots(&self, resources: &Resources) -> Vec<Resource> {
    //     self.recipes
    //         .iter()
    //         .filter_map(|(resource_to_produce, needed_resources)| {
    //             if needed_resources.iter().all(|(k, v)| {
    //                 if let Some(r) = resources.get(k) {
    //                     r >= v
    //                 } else {
    //                     false
    //                 }
    //             }) {
    //                 Some(*resource_to_produce)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }

    // New possible states after eventually creating the robots and consuming the
    // resources. The case waiting is possible.
    // fn new_states(&self, resources: Resources, robots: Robots) -> Vec<(Resources, Robots)> {
    //     let possible_robots = self.possible_robots(&resources);
    //     let mut states = vec![];
    //     // first possibility is doing nothing
    //     let mut new_resources = resources.clone();
    //     for (robot_resource, &number) in robots.iter() {
    //         new_resources
    //             .entry(*robot_resource)
    //             .and_modify(|q| *q += number)
    //             .or_insert(number);
    //     }
    //     states.push((new_resources, robots.clone()));
    //     // Then we can iterate the possible robots
    //     for robot in possible_robots {
    //         let mut new_resources = resources.clone();
    //         let mut new_robots = robots.clone();
    //         self.produce(&mut new_resources, &robot);
    //         new_robots.entry(robot).and_modify(|e| *e += 1).or_insert(1);
    //         states.push((new_resources, new_robots));
    //     }
    //     states
    // }
}

// fn produce_geodes(
//     blueprint: &Blueprint,
//     resources: Resources,
//     robots: Robots,
//     minute: i32,
// ) -> usize {
//     if minute > 24 {
//         if let Some(g) = resources.get(&Resource::Geode) {
//             println!("Possible production: {g}");
//             return blueprint.id * g;
//         } else {
//             return 0;
//         }
//     }
//     let new_states = blueprint.new_states(resources, robots);
//     return new_states
//         .into_iter()
//         .map(|(new_resources, new_robots)| {
//             produce_geodes(&blueprint, new_resources, new_robots, minute + 1)
//         })
//         .max()
//         .unwrap();
// }

fn explore(blueprint: &Blueprint, resources: Resources, robots: Robots, time_left: usize) -> usize {
    // println!("time left {time_left}");
    if time_left <= 1 {
        return 0;
    }
    // if let Some(_) = CACHE.lock().unwrap().iter().find(|((res, rob, t), _)| {
    //     equal(
    //         sorted(res),
    //         sorted(&resources)
    //             .into_iter()
    //             .map(|(_, v)| (v))
    //             .collect::<Vec<_>>(),
    //     ) && equal(
    //         sorted(rob),
    //         sorted(&robots)
    //             .into_iter()
    //             .map(|(_, v)| v)
    //             .collect::<Vec<_>>(),
    //     ) && t >= &time_left
    // }) {
    //     // we already have a solution with the same state and more time left
    //     println!("already found a better solution");
    //     return 0;
    // }
    let mut possible_states = blueprint.next_possible_robots(&resources, &robots, time_left);
    let mut result = 0;
    while let Some((res, rob, t)) = possible_states.pop() {
        result = ((t * rob.get(&Resource::Geode).unwrap_or(&0)) + explore(&blueprint, res, rob, t))
            .max(result);
    }
    // CACHE.lock().unwrap().insert(
    //     {
    //         let resources = [
    //             Resource::Ore,
    //             Resource::Clay,
    //             Resource::Obsidian,
    //             Resource::Geode,
    //         ]
    //         .iter()
    //         .map(|k| *resources.get(k).unwrap_or(&0usize))
    //         .collect();
    //         let robots = [
    //             Resource::Ore,
    //             Resource::Clay,
    //             Resource::Obsidian,
    //             Resource::Geode,
    //         ]
    //         .iter()
    //         .map(|k| *robots.get(k).unwrap_or(&0usize))
    //         .collect();
    //         (resources, robots, time_left)
    //     },
    //     result,
    // );
    result
}

pub fn part1(blueprints: Vec<Blueprint>) -> usize {
    let mut final_state = vec![];
    for blueprint in blueprints.iter() {
        println!("Blueprint {}:\n", blueprint.id);
        let resources: Resources = HashMap::new();
        let mut robots: Robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        final_state.push(explore(&blueprint, resources, robots, 24));
    }
    final_state.iter().sum()
}

#[cfg(test)]
pub mod test {
    use super::*;
    use itertools::assert_equal;
    use itertools::sorted;
    use std::collections::HashMap;

    // #[test]
    // fn test_possible_builds() -> Result<()> {
    //     let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
    //     let mut resources = HashMap::new();
    //     assert_eq!(b.possible_robots(&resources), vec![]);
    //     resources.insert(Resource::Ore, 4);
    //     assert_equal(
    //         sorted(b.possible_robots(&resources)),
    //         vec![Resource::Ore, Resource::Clay],
    //     );
    //     resources.insert(Resource::Clay, 14);
    //     assert_equal(
    //         sorted(b.possible_robots(&resources)),
    //         vec![Resource::Ore, Resource::Clay, Resource::Obsidian],
    //     );
    //     resources.insert(Resource::Obsidian, 7);
    //     assert_equal(
    //         sorted(b.possible_robots(&resources)),
    //         vec![
    //             Resource::Ore,
    //             Resource::Clay,
    //             Resource::Obsidian,
    //             Resource::Geode,
    //         ],
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn test_produce() -> Result<()> {
    //     let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
    //     let mut resources = HashMap::new();
    //     resources.insert(Resource::Ore, 2);
    //     b.produce(&mut resources, &Resource::Clay);
    //     assert_eq!(resources.get(&Resource::Ore), Some(&0usize));
    //     Ok(())
    // }

    // #[test]
    // fn test_new_state() -> Result<()> {
    //     let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
    //     let mut resources = HashMap::new();
    //     resources.insert(Resource::Ore, 4);
    //     let mut robots = HashMap::new();
    //     robots.insert(Resource::Ore, 1);
    //     let new_states = b.new_states(resources, robots);
    //     let mut expected_states = vec![];
    //     let zero_robot = (
    //         HashMap::from_iter([(Resource::Ore, 5)]),
    //         HashMap::from_iter([(Resource::Ore, 1)]),
    //     );
    //     let ore_robot = (
    //         HashMap::from_iter([(Resource::Ore, 0)]),
    //         HashMap::from_iter([(Resource::Ore, 2)]),
    //     );
    //     let clay_robot = (
    //         HashMap::from_iter([(Resource::Ore, 2)]),
    //         HashMap::from_iter([(Resource::Ore, 1), (Resource::Clay, 1)]),
    //     );
    //     expected_states.extend([zero_robot, ore_robot, clay_robot]);
    //     assert_eq!(new_states, expected_states,);
    //     Ok(())
    // }

    #[test]
    fn test_next_possible_robots_basic() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let start_time = 24;
        let resources = HashMap::new();
        let mut robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        // We have one Ore robot and no resources: we can :
        // - produce an Ore robot in 4 minutes. So next time we are ready is 5 minutes
        //   later, we have produced 4 ore, consumed those 4, built a robot and then
        //   produced 2 ores with the 2 robots
        // - wait 2 minute to produce a Clay robot. So, next time we are ready is 3
        //   minutes later, we have produced 2 ore, built a clay robot then produced a
        //   one clay and one ore
        let ore_robot = (
            HashMap::from_iter([
                (Resource::Ore, 2),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0),
            ]),
            HashMap::from_iter([(Resource::Ore, 2)]),
            start_time - 5,
        );
        let clay_robot = (
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 1),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0),
            ]),
            HashMap::from_iter([(Resource::Ore, 1), (Resource::Clay, 1)]),
            start_time - 3,
        );
        let mut possible_next_robots = vec![];
        possible_next_robots.extend([clay_robot, ore_robot]);
        let mut result = b.next_possible_robots(&resources, &robots, start_time);
        // for consistent ordering when comparing
        result.sort_by(|(_, _, a), (_, _, b)| b.cmp(&a));
        assert_eq!(result, possible_next_robots,);
        Ok(())
    }

    #[test]
    fn test_next_possible_robots_more_complex() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let start_time = 24;
        let mut resources = HashMap::new();
        resources.insert(Resource::Ore, 2);
        resources.insert(Resource::Clay, 1);
        resources.insert(Resource::Obsidian, 1);
        let mut robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        robots.insert(Resource::Clay, 1);
        robots.insert(Resource::Obsidian, 1);
        // We can :
        // - produce an Ore robot in 2 minutes. So next time we are ready is 3 minutes
        //   later, we have produced 2 ore, consumed 4, built an ore robot and then
        //   produced 2 ores with the 2 robots. The other robots produce 3 clay and 3
        //   obsidian.
        // - produce a Clay robot right now. So, next time we are ready is 1
        //   minute later, we have produced 1 ore, 1 clay and 1 obsidian, consumed 2 ore
        //   and produced 1 clay more.
        // - produce an obsidian robot in 13 minutes. So next time we are ready is 14
        //   minutes later. We have produced 14 ore, 14 clay, 14 obsidian, built an
        //   obsidian robot, consumed 3 ore and 14 clay, then produced 1 obsidian more.
        // - produce a geode robot in 6 minutes. So next time we are ready is 7 minutes
        //   later, we have produced 7 ore, 7 clay, 7 obsidian, consumed 3 ore and 7
        //   obsidian to build a geode robot, then produced 1 geode.
        let ore_robot = (
            HashMap::from_iter([
                (Resource::Ore, 2),
                (Resource::Clay, 3),
                (Resource::Obsidian, 3),
                (Resource::Geode, 0),
            ]),
            HashMap::from_iter([(Resource::Ore, 2)]),
            start_time - 3,
        );
        let clay_robot = (
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 2),
                (Resource::Obsidian, 1),
                (Resource::Geode, 0),
            ]),
            HashMap::from_iter([(Resource::Ore, 1), (Resource::Clay, 1)]),
            start_time - 1,
        );
        let obsidian_robot = (
            HashMap::from_iter([
                (Resource::Ore, 3 + 14 - 3),
                (Resource::Clay, 1),
                (Resource::Obsidian, 15),
                (Resource::Geode, 0),
            ]),
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 1),
                (Resource::Obsidian, 2),
            ]),
            start_time - 14,
        );
        let geode_robot = (
            HashMap::from_iter([
                (Resource::Ore, 1 + 7 - 3),
                (Resource::Clay, 1),
                (Resource::Obsidian, 1),
                (Resource::Geode, 1),
            ]),
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 1),
                (Resource::Obsidian, 1),
                (Resource::Geode, 1),
            ]),
            start_time - 7,
        );
        let mut possible_next_robots = vec![];
        possible_next_robots.extend([obsidian_robot, geode_robot, clay_robot, ore_robot]);
        let mut result = b.next_possible_robots(&resources, &robots, start_time);
        // for consistent ordering when comparing
        result.sort_by(|(_, _, a), (_, _, b)| a.cmp(&b));
        assert_eq!(result, possible_next_robots,);
        Ok(())
    }

    #[test]
    fn test_explore() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let resources = HashMap::new();
        let mut robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        let result = explore(&b, resources, robots, 24);
        assert_eq!(result, 9);
        Ok(())
    }

    #[test]
    fn test_max_needed_robot() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let mut expected = HashMap::new();
        expected.insert(Resource::Ore, 4);
        expected.insert(Resource::Clay, 14);
        expected.insert(Resource::Obsidian, 7);
        assert_eq!(b.max_needed_robot_per_resource(), expected);
        Ok(())
    }
    // #[test]
    // fn test_produce_geodes() -> Result<()> {
    //     let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
    //     let mut resources = HashMap::new();
    //     resources.insert(Resource::Ore, 4);
    //     let mut robots = HashMap::new();
    //     robots.insert(Resource::Ore, 1);
    //     let result = produce_geodes(&b, resources, robots, 0);
    //     assert_eq!(result, 9);
    //     Ok(())
    // }
}
