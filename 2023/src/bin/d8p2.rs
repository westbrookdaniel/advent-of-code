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

    let (mut entries, exits) = tree.iter().fold((vec![], vec![]), |mut acc, (k, _)| {
        if k.ends_with("A") {
            let exit = k[..k.len() - 1].to_string() + "Z";
            acc.0.push(k.to_string());
            acc.1.push(exit);
        }
        acc
    });

    let mut i = 0;
    while entries.iter().enumerate().any(|(i, c)| exits[i] != *c) {
        let instr = instrs[i % instrs.len()];
        entries = entries
            .par_iter()
            .map(|c| match instr {
                "L" => tree[c].0.clone(),
                "R" => tree[c].1.clone(),
                _ => panic!("Invalid instruction"),
            })
            .collect::<Vec<_>>();
        i += 1;
    }

    println!("{:?}", i);
}
