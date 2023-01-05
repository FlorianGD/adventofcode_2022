use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use petgraph::algo::floyd_warshall;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

type Tunnels = Graph<String, i32, Undirected>;

lazy_static! {
    static ref CACHE_P1: Mutex<HashMap<(Valve, Vec<Valve>, i32), i32>> = Mutex::new(HashMap::new());
    static ref CACHE_P2: Mutex<HashMap<((Valve, Valve), Vec<Valve>, (i32, i32)), i32>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Valve {
    id: NodeIndex,
    name: String,
    flow_rate: i32,
}

fn valve(s: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), alpha1)(s)
}

fn flow_rate(s: &str) -> IResult<&str, i32> {
    map_res(preceded(tag(" has flow rate="), digit1), |i: &str| {
        i.parse::<i32>()
    })(s)
}

fn tunnels(s: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    )(s)
}

fn parse_line(s: &str) -> IResult<&str, (&str, i32, Vec<&str>)> {
    tuple((valve, flow_rate, tunnels))(s)
}

pub fn parse_input(input: &str) -> (Vec<Valve>, HashMap<(NodeIndex, NodeIndex), i32>) {
    // use indoc::indoc;
    // let input = indoc! {
    //         "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    //         Valve BB has flow rate=13; tunnels lead to valves CC, AA
    //         Valve CC has flow rate=2; tunnels lead to valves DD, BB
    //         Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    //         Valve EE has flow rate=3; tunnels lead to valves FF, DD
    //         Valve FF has flow rate=0; tunnels lead to valves EE, GG
    //         Valve GG has flow rate=0; tunnels lead to valves FF, HH
    //         Valve HH has flow rate=22; tunnel leads to valve GG
    //         Valve II has flow rate=0; tunnels lead to valves AA, JJ
    //         Valve JJ has flow rate=21; tunnel leads to valve II"
    // };
    let mut valves = Vec::new();
    let mut g: Tunnels = Graph::new_undirected();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    for line in input.lines() {
        if let Ok((_, (valve_name, flow_rate, edges))) = parse_line(line) {
            let cur;
            let cur_name = valve_name.to_owned();
            if let Vacant(e) = nodes.entry(cur_name.clone()) {
                cur = g.add_node(valve_name.to_string());
                e.insert(cur);
            } else {
                cur = nodes[&cur_name];
            }
            let valve = Valve {
                id: cur,
                name: valve_name.to_string(),
                flow_rate,
            };
            for edge in edges {
                let to;
                let to_name = edge.to_owned();
                if let Vacant(e) = nodes.entry(to_name.clone()) {
                    to = g.add_node(to_name.to_string());
                    e.insert(to);
                } else {
                    to = nodes[&to_name];
                }
                g.add_edge(cur, to, 1);
            }
            valves.push(valve);
        }
    }

    if let Ok(distances) = floyd_warshall(&g, |_| 1) {
        (valves, distances)
    } else {
        panic!()
    }
}

fn solve(
    current: &Valve,
    valves_to_open: &Vec<Valve>,
    distances: &HashMap<(NodeIndex, NodeIndex), i32>,
    time_left: i32,
) -> i32 {
    if CACHE_P1
        .lock()
        .unwrap()
        .contains_key(&(current.clone(), valves_to_open.clone(), time_left))
    {
        return CACHE_P1.lock().unwrap()[&(current.clone(), valves_to_open.clone(), time_left)];
    }
    if time_left <= 1 {
        return 0;
    }
    if valves_to_open.len() == 1 {
        let dest = valves_to_open[0].clone();
        let distance = distances[&(dest.id, current.id)];
        if distance > time_left + 1 {
            0
        } else {
            // opening takes 1 minute
            let time_left = time_left - 1;
            dest.flow_rate * (time_left - distance)
        }
    } else {
        let mut subs = Vec::new();
        for i in 0..valves_to_open.len() {
            let mut valves_to_open = valves_to_open.clone();
            let dest = valves_to_open.remove(i);
            // opening takes 1 minute
            let time_left = time_left - 1;
            let distance = distances[&(dest.id, current.id)];
            let time = time_left - distance;
            if time <= 0 {
                subs.push(0)
            } else {
                let result = dest.flow_rate * time + solve(&dest, &valves_to_open, distances, time);
                subs.push(result);
            }
        }
        let result = subs.into_iter().max().unwrap();
        CACHE_P1
            .lock()
            .unwrap()
            .insert((current.clone(), valves_to_open.clone(), time_left), result);
        result
    }
}

pub fn part1((valves, distances): (Vec<Valve>, HashMap<(NodeIndex, NodeIndex), i32>)) -> i32 {
    if let Some(start) = valves.iter().find(|v| v.name == "AA") {
        let time_left = 30;
        let to_open: Vec<Valve> = valves
            .clone()
            .into_iter()
            .filter(|v| v.flow_rate > 0)
            .collect();

        solve(start, &to_open, &distances, time_left)
    } else {
        panic!()
    }
}

fn solve2(
    (current_me, current_elephant): (&Valve, &Valve),
    valves_to_open: &Vec<Valve>,
    distances: &HashMap<(NodeIndex, NodeIndex), i32>,
    (time_left_me, time_left_elephant): (i32, i32),
) -> i32 {
    if CACHE_P2.lock().unwrap().contains_key(&(
        (current_me.clone(), current_elephant.clone()),
        valves_to_open.clone(),
        (time_left_me, time_left_elephant),
    )) {
        return CACHE_P2.lock().unwrap()[&(
            (current_me.clone(), current_elephant.clone()),
            valves_to_open.clone(),
            (time_left_me, time_left_elephant),
        )];
    }
    if CACHE_P2.lock().unwrap().contains_key(&(
        (current_elephant.clone(), current_me.clone()),
        valves_to_open.clone(),
        (time_left_elephant, time_left_me),
    )) {
        return CACHE_P2.lock().unwrap()[&(
            (current_elephant.clone(), current_me.clone()),
            valves_to_open.clone(),
            (time_left_elephant, time_left_me),
        )];
    }
    if time_left_me <= 1 {
        let result = solve(
            current_elephant,
            valves_to_open,
            distances,
            time_left_elephant,
        );
        CACHE_P2.lock().unwrap().insert(
            (
                (current_me.clone(), current_elephant.clone()),
                valves_to_open.clone(),
                (time_left_me, time_left_elephant),
            ),
            result,
        );
        return result;
    } else if time_left_elephant <= 1 {
        let result = solve(current_me, valves_to_open, distances, time_left_me);
        CACHE_P2.lock().unwrap().insert(
            (
                (current_me.clone(), current_elephant.clone()),
                valves_to_open.clone(),
                (time_left_me, time_left_elephant),
            ),
            result,
        );
        return result;
    } else if valves_to_open.len() == 1 {
        let dest = valves_to_open[0].clone();
        let d_me = distances[&(dest.id, current_me.id)];
        let d_elephant = distances[&(dest.id, current_elephant.id)];
        if d_me > time_left_me + 1 {
            // unreachable for me
            if d_elephant > time_left_elephant + 1 {
                CACHE_P2.lock().unwrap().insert(
                    (
                        (current_me.clone(), current_elephant.clone()),
                        valves_to_open.clone(),
                        (time_left_me, time_left_elephant),
                    ),
                    0,
                );
                0
            } else {
                let result = dest.flow_rate * (time_left_elephant - 1 - d_elephant);
                CACHE_P2.lock().unwrap().insert(
                    (
                        (current_me.clone(), current_elephant.clone()),
                        valves_to_open.clone(),
                        (time_left_me, time_left_elephant),
                    ),
                    result,
                );
                result
            }
        } else if d_elephant > time_left_elephant + 1 {
            // unreachable for elephant
            let result = dest.flow_rate * (time_left_me - 1 - d_me);
            CACHE_P2.lock().unwrap().insert(
                (
                    (current_me.clone(), current_elephant.clone()),
                    valves_to_open.clone(),
                    (time_left_me, time_left_elephant),
                ),
                result,
            );
            result
        } else {
            let result = (dest.flow_rate * (time_left_elephant - 1 - d_elephant))
                .max(dest.flow_rate * (time_left_me - 1 - d_me));
            CACHE_P2.lock().unwrap().insert(
                (
                    (current_me.clone(), current_elephant.clone()),
                    valves_to_open.clone(),
                    (time_left_me, time_left_elephant),
                ),
                result,
            );
            result
        }
    } else {
        let mut subs = Vec::new();
        for i in 0..valves_to_open.len() {
            let mut valves_to_open = valves_to_open.clone();
            let dest_me = valves_to_open.remove(i);
            for j in 0..valves_to_open.len() {
                let mut valves_to_open = valves_to_open.clone();
                let dest_elephant = valves_to_open.remove(j);
                // opening takes 1 minute
                let time_left_me = time_left_me - 1;
                let time_left_elephant = time_left_elephant - 1;
                let d_me = distances[&(dest_me.id, current_me.id)];
                let d_elephant = distances[&(dest_elephant.id, current_elephant.id)];
                let time_left_me = time_left_me - d_me;
                let time_left_elephant = time_left_elephant - d_elephant;
                let result;
                if time_left_me <= 0 {
                    if time_left_elephant <= 0 {
                        result = 0;
                    } else {
                        result = dest_elephant.flow_rate * time_left_elephant
                            + solve2(
                                (current_me, &dest_elephant),
                                &valves_to_open,
                                distances,
                                (time_left_me, time_left_elephant),
                            );
                    }
                } else if time_left_elephant <= 0 {
                    let partial_result = solve2(
                        (&dest_me, current_elephant),
                        &valves_to_open,
                        distances,
                        (time_left_me, time_left_elephant),
                    );
                    CACHE_P2.lock().unwrap().insert(
                        (
                            (dest_me.clone(), current_elephant.clone()),
                            valves_to_open.clone(),
                            (time_left_me, time_left_elephant),
                        ),
                        partial_result,
                    );
                    result = dest_me.flow_rate * time_left_me + partial_result;
                } else {
                    let partial_result = solve2(
                        (&dest_me, &dest_elephant),
                        &valves_to_open,
                        distances,
                        (time_left_me, time_left_elephant),
                    );
                    CACHE_P2.lock().unwrap().insert(
                        (
                            (dest_me.clone(), current_elephant.clone()),
                            valves_to_open.clone(),
                            (time_left_me, time_left_elephant),
                        ),
                        partial_result,
                    );
                    result = dest_me.flow_rate * time_left_me
                        + dest_elephant.flow_rate * time_left_elephant
                        + partial_result;
                }

                subs.push(result);
            }
        }

        let result = subs.into_iter().max().unwrap_or(0);
        CACHE_P2.lock().unwrap().insert(
            (
                (current_me.clone(), current_elephant.clone()),
                valves_to_open.clone(),
                (time_left_me, time_left_elephant),
            ),
            result,
        );
        result
    }
}

pub fn part2((valves, distances): (Vec<Valve>, HashMap<(NodeIndex, NodeIndex), i32>)) -> i32 {
    if let Some(start) = valves.iter().find(|v| v.name == "AA") {
        let time_left = 26;
        let to_open: Vec<Valve> = valves
            .clone()
            .into_iter()
            .filter(|v| v.flow_rate > 0)
            .collect();

        solve2((start, start), &to_open, &distances, (time_left, time_left))
    } else {
        panic!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;
    use petgraph::graph::NodeIndex;
    lazy_static! {
        static ref CACHE_P1: Mutex<HashMap<(Valve, Vec<Valve>, i32), i32>> =
            Mutex::new(HashMap::new());
        static ref CACHE_P2: Mutex<HashMap<((Valve, Valve), Vec<Valve>, (i32, i32)), i32>> =
            Mutex::new(HashMap::new());
    }
    #[test]
    fn test_valve() {
        assert_eq!(valve("Valve BB"), Ok(("", "BB")));
        assert_eq!(valve("Valve CC "), Ok((" ", "CC")));
    }

    #[test]
    fn test_flow_rate() {
        assert_eq!(flow_rate(" has flow rate=13"), Ok(("", 13)));
        assert_eq!(flow_rate(" has flow rate=0"), Ok(("", 0)));
    }

    #[test]
    fn test_tunnels() {
        assert_eq!(
            tunnels("; tunnels lead to valves CC, AA"),
            Ok(("", vec!["CC", "AA"]))
        );
        assert_eq!(tunnels("; tunnel leads to valve GG"), Ok(("", vec!["GG"])));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            Ok(("", ("AA", 0, vec!["DD", "II", "BB"])))
        );
        assert_eq!(
            parse_line("Valve JJ has flow rate=21; tunnel leads to valve II"),
            Ok(("", ("JJ", 21, vec!["II"])))
        );
    }
    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "Valve AA has flow rate=0; tunnels lead to valves CC, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=5; tunnels lead to valves AA, BB
        "};
        let (valves, distances) = parse_input(input);
        assert_eq!(
            valves,
            vec![
                Valve {
                    id: NodeIndex::new(0),
                    name: "AA".into(),
                    flow_rate: 0,
                },
                Valve {
                    id: NodeIndex::new(2),
                    name: "BB".into(),
                    flow_rate: 13,
                },
                Valve {
                    id: NodeIndex::new(1),
                    name: "CC".into(),
                    flow_rate: 5,
                },
            ]
        );
        let mut expected_distances = HashMap::new();
        expected_distances.insert((NodeIndex::<u32>::new(0), NodeIndex::<u32>::new(2)), 1i32);
        expected_distances.insert((NodeIndex::new(2), NodeIndex::new(0)), 1);
        expected_distances.insert((NodeIndex::new(2), NodeIndex::new(1)), 1);
        expected_distances.insert((NodeIndex::new(2), NodeIndex::new(2)), 0);
        expected_distances.insert((NodeIndex::new(0), NodeIndex::new(0)), 0);
        expected_distances.insert((NodeIndex::new(1), NodeIndex::new(0)), 1);
        expected_distances.insert((NodeIndex::new(1), NodeIndex::new(1)), 0);
        expected_distances.insert((NodeIndex::new(1), NodeIndex::new(2)), 1);
        expected_distances.insert((NodeIndex::new(0), NodeIndex::new(1)), 1);
        assert_eq!(distances, expected_distances);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
                Valve BB has flow rate=13; tunnels lead to valves CC, AA
                Valve CC has flow rate=2; tunnels lead to valves DD, BB
                Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
                Valve EE has flow rate=3; tunnels lead to valves FF, DD
                Valve FF has flow rate=0; tunnels lead to valves EE, GG
                Valve GG has flow rate=0; tunnels lead to valves FF, HH
                Valve HH has flow rate=22; tunnel leads to valve GG
                Valve II has flow rate=0; tunnels lead to valves AA, JJ
                Valve JJ has flow rate=21; tunnel leads to valve II"
        };
        let (valves, distances) = parse_input(input);
        let result = part1((valves, distances));
        assert_eq!(result, 1651)
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
                Valve BB has flow rate=13; tunnels lead to valves CC, AA
                Valve CC has flow rate=2; tunnels lead to valves DD, BB
                Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
                Valve EE has flow rate=3; tunnels lead to valves FF, DD
                Valve FF has flow rate=0; tunnels lead to valves EE, GG
                Valve GG has flow rate=0; tunnels lead to valves FF, HH
                Valve HH has flow rate=22; tunnel leads to valve GG
                Valve II has flow rate=0; tunnels lead to valves AA, JJ
                Valve JJ has flow rate=21; tunnel leads to valve II"
        };
        let (valves, distances) = parse_input(input);
        let result = part2((valves, distances));
        assert_eq!(result, 1707)
    }
}
