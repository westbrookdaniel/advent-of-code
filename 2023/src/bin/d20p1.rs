#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Module {
    FlipFlop(String),
    Conjunction(String),
}

#[derive(Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Node {
    data: Module,
    to: Vec<String>,
}

#[derive(Debug)]
struct Signal {
    name: String,
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
                        name: s.to_string(),
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

    for node in graph.iter() {
        println!("{:?}", node);
    }
    println!("{:?}", broadcaster);
    println!();

    let initial_n = 1 + broadcaster.len();
    let mut signals = broadcaster;
    let mut i = 0;

    let mut conjunction_state: HashMap<String, HashMap<String, Option<Pulse>>> = HashMap::new();
    let mut flipflop_state: HashMap<String, bool> = HashMap::new();

    while signals.len() > 0 {
        let signal = signals.pop_front().unwrap();
        let mut node = find_node(&graph, &signal.name).unwrap();

        let new_pulse = match &node.data {
            Module::FlipFlop(name) => match signal.pulse {
                Pulse::Low => {
                    let mut is_on = flipflop_state.entry(name.clone()).or_insert(false);

                    if *is_on {
                        is_on = &mut false;
                        Some(Pulse::Low)
                    } else {
                        is_on = &mut true;
                        Some(Pulse::High)
                    }
                }
                _ => None,
            },
            Module::Conjunction(name) => {
                let mut state = conjunction_state
                    .entry(name.clone())
                    .or_insert(HashMap::new());

                if state.len() != node.to.len() {
                    None
                } else {
                    None
                }
            }
        };

        i += 1;
    }

    let n = initial_n + i;

    println!("{}", n);
}

fn find_node<'a>(graph: &'a Vec<Node>, name: &str) -> Option<&'a Node> {
    graph.iter().find(|node| match &node.data {
        Module::FlipFlop(n) => n == name,
        Module::Conjunction(n) => n == name,
    })
}
