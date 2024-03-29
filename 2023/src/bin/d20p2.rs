#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

use num::integer::lcm;

#[derive(Debug)]
enum Module {
    FlipFlop(String),
    Conjunction(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Node {
    data: Module,
    to: Vec<String>,
}

#[derive(Debug, Clone)]
struct Signal {
    to: String,
    from: String,
    pulse: Pulse,
}

fn main() {
    let input = std::fs::read_to_string("src/input/d20p1.txt").unwrap();

    let mut broadcaster = VecDeque::new();

    let graph = input
        .trim()
        .lines()
        .filter(|line| {
            if line.starts_with("broadcast") {
                let (_, to) = line.split_once(" -> ").unwrap();
                let to = to
                    .trim()
                    .split(", ")
                    .map(|s| Signal {
                        to: s.to_string(),
                        from: "broadcast".to_string(),
                        pulse: Pulse::Low,
                    })
                    .collect::<Vec<_>>();
                broadcaster.extend(to);
                false
            } else {
                true
            }
        })
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let mut chars = from.chars();
            let module = chars.next().unwrap();
            let from = chars.collect::<String>();
            let from = match module {
                '%' => Module::FlipFlop(from),
                '&' => Module::Conjunction(from),
                _ => panic!("unknown module"),
            };
            let to = to
                .trim()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            Node { data: from, to }
        })
        .collect::<Vec<_>>();

    let mut conjunction_state: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
    let mut flipflop_state: HashMap<String, bool> = HashMap::new();

    let mut emitters = HashMap::new();

    let mut i = 0;

    loop {
        i += 1;

        let from_xm = get_from(&find_node(&graph, "xm").unwrap(), &graph);

        let lcm_emitters = broadcast(
            &graph,
            &broadcaster,
            &mut conjunction_state,
            &mut flipflop_state,
        );

        // merge lcm_emitters into emitters
        merge(&mut emitters, lcm_emitters, i);

        if emitters.len() == from_xm.len() {
            break;
        }
    }

    let out = emitters
        .values()
        .map(|(_, i)| i)
        .fold(1, |acc, x| lcm(acc, *x));

    println!("{:?}", out);
}

fn merge(
    emitters: &mut HashMap<String, (Pulse, usize)>,
    lcm_emitters: HashMap<String, (Pulse, usize)>,
    i: usize,
) {
    for (name, (pulse, _)) in lcm_emitters {
        if let None = emitters.get(&name) {
            emitters.insert(name, (pulse, i));
        }
    }
}

fn find_node<'a>(graph: &'a Vec<Node>, name: &str) -> Option<&'a Node> {
    graph.iter().find(|node| match &node.data {
        Module::FlipFlop(n) => n == name,
        Module::Conjunction(n) => n == name,
    })
}

fn get_from(node: &Node, graph: &Vec<Node>) -> Vec<String> {
    let get_name = |node: &Node| match &node.data {
        Module::FlipFlop(n) => n.clone(),
        Module::Conjunction(n) => n.clone(),
    };

    let name = get_name(node);

    graph.iter().fold(Vec::new(), |mut acc, n| {
        if n.to.contains(&name) {
            acc.push(get_name(n).to_string());
        }
        acc
    })
}

fn pulse_from_conjunction(state: &HashMap<String, Pulse>, from: Vec<String>) -> Pulse {
    let from_pulses = from
        .iter()
        .map(|name| state.get(name).unwrap_or(&Pulse::Low))
        .collect::<Vec<_>>();

    if from_pulses.iter().all(|pulse| **pulse == Pulse::High) {
        Pulse::Low
    } else {
        Pulse::High
    }
}

fn broadcast(
    graph: &Vec<Node>,
    broadcaster: &VecDeque<Signal>,
    conjunction_state: &mut HashMap<String, HashMap<String, Pulse>>,
    flipflop_state: &mut HashMap<String, bool>,
) -> HashMap<String, (Pulse, usize)> {
    let mut signals = broadcaster.clone();
    let mut h = 0;
    let mut l = 1;
    let mut lcm_emitters = HashMap::new();

    while signals.len() > 0 {
        let signal = signals.pop_front().unwrap();
        match signal.pulse {
            Pulse::High => h += 1,
            Pulse::Low => l += 1,
        }

        if signal.to == "xm" && signal.pulse == Pulse::High {
            lcm_emitters.insert(signal.from.clone(), (signal.pulse, h + l));
        }

        // This check is for the case where a signal is sent to a node that doesn't exist.
        // Used in examples
        let node = find_node(&graph, &signal.to);
        if node.is_none() {
            continue;
        }
        let node = node.unwrap();

        let new_pulse = match &node.data {
            Module::FlipFlop(name) => match signal.pulse {
                Pulse::Low => {
                    let is_on = flipflop_state.entry(name.clone()).or_insert(false);

                    if *is_on {
                        flipflop_state.insert(name.clone(), false);
                        Some(Pulse::Low)
                    } else {
                        flipflop_state.insert(name.clone(), true);
                        Some(Pulse::High)
                    }
                }
                _ => None, // broadcaster.len();,
            },
            Module::Conjunction(name) => {
                let state = conjunction_state
                    .entry(name.clone())
                    .or_insert(HashMap::new());

                state.insert(signal.from.clone(), signal.pulse);

                let from = get_from(&node, &graph);

                Some(pulse_from_conjunction(state, from))
            }
        };

        if let Some(new_pulse) = new_pulse {
            for name in node.to.iter() {
                // println!("{:?}", node);
                // println!("{} -{:?}> {}", signal.to, new_pulse, name);
                // println!();
                let signal = Signal {
                    to: name.clone(),
                    from: signal.to.clone(),
                    pulse: new_pulse,
                };
                signals.push_back(signal);
            }
        }
    }

    lcm_emitters
}
