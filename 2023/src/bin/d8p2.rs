use num::integer::lcm;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("src/input/d8p1.txt").unwrap();

    let (instrs, tree) = input.split_at(input.find("\n\n").unwrap());

    // create binary tree, where each line: AAA = (BBB, CCC)
    let tree = tree
        .trim()
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" = ").unwrap();
            let (left, right) = to.split_once(", ").unwrap();
            let left = left.strip_prefix('(').unwrap();
            let right = right.strip_suffix(')').unwrap();
            (from.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<std::collections::HashMap<_, _>>();

    let instrs = instrs
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let entries = tree.iter().fold(vec![], |mut acc, (k, _)| {
        if k.ends_with("A") {
            acc.push(k.to_string());
        }
        acc
    });

    let mins = entries
        .par_iter()
        .map(|entry| {
            let mut i = 0;
            let mut curr = entry.as_str();
            let tree = tree.clone();

            while !curr.ends_with('Z') {
                let instr = instrs[i % instrs.len()];
                curr = match instr {
                    "L" => tree[curr].0.as_str(),
                    "R" => tree[curr].1.as_str(),
                    _ => curr,
                };
                i += 1;
            }

            i
        })
        .collect::<Vec<_>>();

    let min = mins.iter().fold(1, |acc, x| lcm(acc, *x));

    println!("{:?}", min);
}
