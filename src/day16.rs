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
use std::collections::HashMap;

type Tunnels = Graph<String, i32, Undirected>;

#[derive(Debug, Clone, PartialEq)]
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
            if nodes.contains_key(&cur_name) {
                cur = nodes[&cur_name];
            } else {
                cur = g.add_node(valve_name.to_string());
                nodes.insert(cur_name, cur);
            }
            let valve = Valve {
                id: cur,
                name: valve_name.to_string(),
                flow_rate,
            };
            for edge in edges {
                let to;
                let to_name = edge.to_owned();
                if nodes.contains_key(&to_name) {
                    to = nodes[&to_name];
                } else {
                    to = g.add_node(to_name.to_string());
                    nodes.insert(to_name, to);
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
    if time_left <= 1 {
        return 0;
    }
    if valves_to_open.len() == 1 {
        let dest = valves_to_open[0].clone();
        let distance = distances[&(dest.id, current.id.clone())];
        if distance > time_left + 1 {
            return 0;
        } else {
            // opening takes 1 minute
            let time_left = time_left - 1;
            return dest.flow_rate * (time_left - distance);
        }
    } else {
        let mut subs = Vec::new();
        for i in 0..valves_to_open.len() {
            let mut valves_to_open = valves_to_open.clone();
            let dest = valves_to_open.remove(i);
            // opening takes 1 minute
            let time_left = time_left - 1;
            let distance = distances[&(dest.id.clone(), current.id.clone())];
            let time = time_left - distance;
            if time <= 0 {
                subs.push(0)
            } else {
                let result =
                    dest.flow_rate * time + solve(&dest, &valves_to_open, &distances, time);
                subs.push(result);
            }
        }
        subs.into_iter().max().unwrap()
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

        solve(&start, &to_open, &distances, time_left)
    } else {
        panic!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use petgraph::graph::NodeIndex;

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
}
