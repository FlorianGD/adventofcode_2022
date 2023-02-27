use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use derivative::Derivative;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::{delimited, tuple};
use nom::IResult;
// use once_cell::sync::Lazy;
// use std::sync::Mutex;

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

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn parse_input(input: &str) -> Result<Vec<Blueprint>> {
    // use indoc::indoc;
    // let input = indoc! {"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."};
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
                        [(Resource::Ore, ore_robot_cost_in_ore)]
                            .into_iter()
                            .collect(),
                    ),
                    (
                        Resource::Clay,
                        [(Resource::Ore, clay_robot_cost_in_ore)]
                            .into_iter()
                            .collect(),
                    ),
                    (
                        Resource::Obsidian,
                        [
                            (Resource::Ore, obs_robot_cost_in_ore),
                            (Resource::Clay, obs_robot_cost_in_clay),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    (
                        Resource::Geode,
                        [
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
}

#[derive(Derivative, Clone, PartialEq, Eq)]
#[derivative(Debug)]
struct State {
    #[derivative(Debug = "ignore")]
    blueprint: Blueprint,
    resources: Resources,
    robots: Robots,
    time_left: usize,
}

impl State {
    fn from_blueprint(b: Blueprint) -> Self {
        let mut robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        State {
            blueprint: b,
            resources: HashMap::new(),
            robots,
            time_left: 24,
        }
    }

    /// Can I create a robot producing this resource, and if yes, when?
    fn minutes_until_robot_ready(&self, robot: Resource) -> Option<usize> {
        if let Some(needed_resources) = self.blueprint.recipes.get(&robot) {
            if let Some(maybe_time) = needed_resources
                .iter()
                .map(|(resource, quantity)| {
                    let number_we_have = self.resources.get(&resource).unwrap_or(&0);
                    if number_we_have >= quantity {
                        Some(0usize)
                    } else {
                        let number_we_need = quantity - number_we_have;

                        if let Some(number_by_minute) = self.robots.get(&resource) {
                            if number_by_minute == &0 {
                                None
                            } else {
                                // could not make `.div_ceil` work, it is behind a
                                // feature gate but it does not seem available.
                                let q = number_we_need as f32 / *number_by_minute as f32;
                                Some(q.ceil() as usize)
                            }
                        } else {
                            None
                        }
                    }
                })
                .collect::<Option<Vec<usize>>>()
            {
                Some(*maybe_time.iter().max().unwrap())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn wait(&mut self, minutes: usize) {
        for r in vec![Resource::Ore, Resource::Clay, Resource::Obsidian] {
            let q1 = self.resources.entry(r).or_insert(0);
            *q1 += minutes * self.robots.get(&r).unwrap_or(&0);
        }
        self.time_left -= minutes;
    }

    fn next_robots_to_consider(&self) -> Vec<Resource> {
        if self.time_left <= 1 {
            // if time left is 1, we will not build any more robots as they will not be
            // ready.
            return vec![];
        }
        // If time left is 2, we will only consider Geode robots as any other robot
        // will not lead to more Geode produced before it is over
        let mut robots_to_consider: Vec<Resource> = vec![Resource::Geode];
        if self.time_left > 2 {
            robots_to_consider.extend(
                self.blueprint
                    .max_needed_robot_per_resource()
                    .iter()
                    .filter_map(|(robot, max_needed)| {
                        self.robots
                            .get(robot)
                            .or(Some(&0))
                            .and_then(|quantity_robot| {
                                if quantity_robot < max_needed {
                                    if let Some(&q_resources) =
                                        self.resources.get(robot).or(Some(&0))
                                    {
                                        if q_resources > max_needed + max_needed / 4 {
                                            None
                                        } else {
                                            Some(*robot)
                                        }
                                    } else {
                                        Some(*robot)
                                    }
                                } else {
                                    None
                                }
                            })
                    }),
            );
        }
        robots_to_consider
    }

    /// Let's see how long we need to wait until we can build one kind of robot and jump
    /// directly to it.
    /// Returns: a vec of what is built and the associated state.
    fn next_states(&self) -> Vec<(Resource, State)> {
        let mut possible = vec![];
        for robot in self.next_robots_to_consider() {
            match self.minutes_until_robot_ready(robot) {
                None => continue, // cannot build robot with current resources
                Some(time_when_ready) => {
                    if time_when_ready >= self.time_left - 1 {
                        continue;
                    }
                    let mut new_state = self.clone();

                    let needed_resources = self.blueprint.recipes.get(&robot).unwrap();
                    // update the resources in `time_when_ready+1` minutes
                    new_state.wait(time_when_ready + 1);
                    // remove the resources we needed to build the robot
                    for (resource, needed_qty) in needed_resources {
                        new_state
                            .resources
                            .entry(*resource)
                            .and_modify(|q| *q -= needed_qty);
                    }
                    if robot != Resource::Geode {
                        // no need to consider the Geode robot, we compute directly the
                        // amount available at the end
                        new_state
                            .robots
                            .entry(robot)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }

                    possible.push((robot, new_state));
                }
            }
        }
        possible
    }

    fn explore(&self) -> usize {
        if self.time_left <= 1 {
            return 0;
        }
        // if let Some(_) = CACHE.lock().unwrap().iter().find(|((res, rob, t), _)| {
        //     equal(
        //         sorted(res),
        //         sorted(&resources)
        //             .into_iterP()
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
        self.next_states()
            .into_iter()
            .map(|(new_robot, state)| {
                // println!("{indent}################  Building {new_robot:?}  ################");

                let mut result = 0;
                if new_robot == Resource::Geode {
                    result += state.time_left;
                }
                // println!(
                //     "{indent}New state: resources: {:?}\n{indent}robots: {:?}",
                //     state.resources, state.robots
                // );
                result + state.explore()
            })
            .max()
            .unwrap_or(0)

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
        // result
    }
}

pub fn part1(blueprints: Vec<Blueprint>) -> usize {
    let mut final_state = vec![];
    for blueprint in blueprints {
        let id = blueprint.id;
        let state = State::from_blueprint(blueprint);
        final_state.push(id * state.explore());
    }
    final_state.iter().sum()
}

pub fn part2(mut blueprints: Vec<Blueprint>) -> usize {
    let mut final_state = vec![];
    blueprints.truncate(3);
    for blueprint in blueprints {
        let mut state = State::from_blueprint(blueprint.clone());
        state.time_left = 32;

        final_state.push(state.explore());
    }
    final_state.iter().product()
}

#[cfg(test)]
pub mod test {
    use super::*;
    // use itertools::assert_equal;
    // use itertools::sorted;
    use std::collections::{HashMap, HashSet};

    #[test]
    // #[ignore]
    fn test_explore() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let state = State::from_blueprint(b);
        let result = state.explore();
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

    #[test]
    fn test_next_states_basic() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let state = State::from_blueprint(b.clone());
        let start_time = state.time_left;
        // We have one Ore robot and no resources: we can :
        // - produce an Ore robot in 4 minutes. So next time we are ready is 5 minutes
        //   later, we have produced 5 ore, those 4, and built a robot
        // - wait 2 minute to produce a Clay robot. So, next time we are ready is 3
        //   minutes later, we have produced 3 ore, and built a clay robot
        let ore_state = State {
            blueprint: b.clone(),
            resources: HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
            ]),
            robots: HashMap::from_iter([(Resource::Ore, 2)]),
            time_left: start_time - 5,
        };
        let clay_state = State {
            blueprint: b.clone(),
            resources: HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
            ]),
            robots: HashMap::from_iter([(Resource::Ore, 1), (Resource::Clay, 1)]),
            time_left: start_time - 3,
        };

        let mut result = state.next_states();
        // for consistent ordering when comparing
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));

        // ore robot built
        assert_eq!(result[0].0, Resource::Ore);
        assert_eq!(result[0].1, ore_state);

        // clay robot built
        assert_eq!(result[1].0, Resource::Clay);
        assert_eq!(result[1].1, clay_state);
        Ok(())
    }

    #[test]
    fn test_next_states_basic_step_by_step() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let state = State::from_blueprint(b.clone());
        let mut total_geodes = 0;

        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("initial state {state:?}\n");
        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;

        total_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 24 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        println!("\n{result:?}");
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;
        total_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("\nstate {state:?}");
        println!("== Minute {:2} ==", 24 - state.time_left);

        // finished
        let result = state.next_states();
        println!("\n{result:?}");

        assert_eq!(total_geodes, 9);
        Ok(())
    }

    #[test]
    fn test_part2_step_by_step() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let mut state = State::from_blueprint(b.clone());
        state.time_left = 32;

        let mut num_geodes = 0;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("initial state {state:?}\n");

        // build ore robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[0].0, Resource::Ore);
        let state = &result[0].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build clay robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[1].0, Resource::Clay);
        let state = &result[1].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build obsidian robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Obsidian);
        let state = &result[2].1;
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[3].0, Resource::Geode);
        let state = &result[3].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Geode);
        let state = &result[2].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Geode);
        let state = &result[2].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[2].0, Resource::Geode);
        let state = &result[2].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // build geode robot
        let mut result = state.next_states();
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));
        assert_eq!(result[0].0, Resource::Geode);
        let state = &result[0].1;
        num_geodes += state.time_left;
        println!("Geode produced: {}", state.time_left);
        println!("== Minute {:2} ==", 32 - state.time_left);
        println!("state {state:?}\n");

        // finished
        let result = state.next_states();
        println!("\n{result:?}");
        assert!(result.is_empty());

        assert_eq!(num_geodes, 56);
        Ok(())
    }

    #[test]
    fn test_compute_resources() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let mut state = State::from_blueprint(b);

        state.robots.insert(Resource::Ore, 2);
        state.robots.insert(Resource::Clay, 1);
        let time = 10;
        state.wait(time);
        let mut expected = HashMap::new();
        expected.insert(Resource::Ore, 2 * time);
        expected.insert(Resource::Clay, time);
        expected.insert(Resource::Obsidian, 0);
        assert_eq!(state.resources, expected);
        assert_eq!(state.time_left, 24 - time);
        Ok(())
    }

    #[test]
    fn test_next_robots_to_consider() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let state = State::from_blueprint(b);
        let robots: HashSet<Resource> = HashSet::from_iter(state.next_robots_to_consider());
        let expected: HashSet<Resource> = HashSet::from_iter([
            Resource::Obsidian,
            Resource::Geode,
            Resource::Ore,
            Resource::Clay,
        ]);
        assert_eq!(robots, expected);
        Ok(())
    }

    #[test]
    fn test_minutes_until_robot_ready() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let mut s = State::from_blueprint(b);
        s.resources.insert(Resource::Ore, 2);
        s.robots.insert(Resource::Clay, 1);

        // We have one Ore robot, one Clay and 2 Ores already. We can build a Clay Robot
        // right now
        assert_eq!(s.minutes_until_robot_ready(Resource::Clay), Some(0));
        // We can wait 2 minutes to build an Ore Robot
        assert_eq!(s.minutes_until_robot_ready(Resource::Ore), Some(2));
        // We can wait 14 minutes to build an obsidian robot
        assert_eq!(s.minutes_until_robot_ready(Resource::Obsidian), Some(14));
        // We cannot build a Geode robot, as we do not have an obsidian robot
        assert_eq!(s.minutes_until_robot_ready(Resource::Geode), None);
        Ok(())
    }

    #[test]
    fn test_minutes_until_robot_ready_2() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let mut s = State::from_blueprint(b);
        s.resources.insert(Resource::Ore, 3);
        s.resources.insert(Resource::Clay, 16);
        s.robots.insert(Resource::Ore, 3);
        s.robots.insert(Resource::Clay, 2);

        // We have 3 Ore robot, 2 Clay and 2 Ores already. We can build a Clay Robot
        // right now
        assert_eq!(s.minutes_until_robot_ready(Resource::Clay), Some(0));
        // We can wait 1 minutes to build an Ore Robot
        assert_eq!(s.minutes_until_robot_ready(Resource::Ore), Some(1));
        // We can wait 1 minutes to build an obsidian robot
        assert_eq!(s.minutes_until_robot_ready(Resource::Obsidian), Some(0));
        // We cannot build a Geode robot, as we do not have an obsidian robot
        assert_eq!(s.minutes_until_robot_ready(Resource::Geode), None);
        Ok(())
    }

    #[test]
    fn test_next_states_more_complex() -> Result<()> {
        let b: Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse()?;
        let mut state = State::from_blueprint(b.clone());
        let start_time = 24;
        let mut resources = HashMap::new();
        resources.insert(Resource::Ore, 2);
        resources.insert(Resource::Clay, 1);
        resources.insert(Resource::Obsidian, 1);
        state.resources = resources;

        let mut robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        robots.insert(Resource::Clay, 1);
        robots.insert(Resource::Obsidian, 1);
        state.robots = robots;

        let mut result = state.next_states();
        // for consistent ordering when comparing
        result.sort_by(|(a, _), (b, _)| a.cmp(&b));

        // We can :
        // - produce an Ore robot in 2 minutes. So next time we are ready is 3 minutes
        //   later, we have produced 3 ore, consumed 4, built an ore robot. The other
        //   robots produce 3 clay and 3 obsidian.
        let ore_state = State {
            blueprint: b.clone(),
            resources: HashMap::from_iter([
                (Resource::Ore, 2 + 3 - 4),
                (Resource::Clay, 1 + 3),
                (Resource::Obsidian, 1 + 3),
            ]),
            robots: HashMap::from_iter([
                (Resource::Ore, 2),
                (Resource::Clay, 1),
                (Resource::Obsidian, 1),
            ]),
            time_left: start_time - 3,
        };
        // ore robot built
        assert_eq!(result[0].0, Resource::Ore);
        assert_eq!(result[0].1.resources, ore_state.resources);
        assert_eq!(result[0].1.robots, ore_state.robots);
        assert_eq!(result[0].1.time_left, ore_state.time_left);
        // - produce a Clay robot right now. So, next time we are ready is 1 minute
        //   later, we have produced 1 ore, 1 clay and 1 obsidian, consumed 2 ore.
        let (clay_robot_resources, clay_robot_robots, clay_robot_time_left) = (
            HashMap::from_iter([
                (Resource::Ore, 2 + 1 - 2),
                (Resource::Clay, 1 + 1),
                (Resource::Obsidian, 2),
            ]),
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 2),
                (Resource::Obsidian, 1),
            ]),
            start_time - 1,
        );
        // clay robot built
        assert_eq!(result[1].0, Resource::Clay);
        assert_eq!(result[1].1.resources, clay_robot_resources);
        assert_eq!(result[1].1.robots, clay_robot_robots);
        assert_eq!(result[1].1.time_left, clay_robot_time_left);
        // - produce an obsidian robot in 13 minutes. So next time we are ready is 14
        //   minutes later. We have produced 14 ore, 14 clay, 14 obsidian, built an
        //   obsidian robot, consumed 3 ore and 14 clay.
        let (obsidian_robot_resources, obsidian_robot_robots, obsidian_robot_time_left) = (
            HashMap::from_iter([
                (Resource::Ore, 2 + 14 - 3),
                (Resource::Clay, 1 + 14 - 14),
                (Resource::Obsidian, 1 + 13 + 1),
            ]),
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 1),
                (Resource::Obsidian, 2),
            ]),
            start_time - 14,
        );
        // obsidian robot built
        assert_eq!(result[2].0, Resource::Obsidian);
        assert_eq!(result[2].1.resources, obsidian_robot_resources);
        assert_eq!(result[2].1.robots, obsidian_robot_robots);
        assert_eq!(result[2].1.time_left, obsidian_robot_time_left);
        // - produce a geode robot in 6 minutes. So next time we are ready is 7 minutes
        //   later, we have produced 7 ore, 7 clay, 7 obsidian, consumed 2 ore and 7
        //   obsidian to build a geode robot.
        let (geode_robot_resources, geode_robot_robots, geode_robot_time_left) = (
            HashMap::from_iter([
                (Resource::Ore, 2 + 7 - 2),
                (Resource::Clay, 1 + 7),
                (Resource::Obsidian, 1 + 7 - 7),
            ]),
            HashMap::from_iter([
                (Resource::Ore, 1),
                (Resource::Clay, 1),
                (Resource::Obsidian, 1),
            ]),
            start_time - 7,
        );
        // geode robot built
        assert_eq!(result[3].0, Resource::Geode);
        assert_eq!(result[3].1.resources, geode_robot_resources);
        assert_eq!(result[3].1.robots, geode_robot_robots);
        assert_eq!(result[3].1.time_left, geode_robot_time_left);

        Ok(())
    }
}
