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
            (from, (left, right))
        })
        .collect::<std::collections::HashMap<_, _>>();

    let instrs = instrs
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut data = (0, "AAA");

    while data.1 != "ZZZ" {
        let instr = instrs[data.0 % instrs.len()];
        let new = match instr {
            "L" => (data.0 + 1, tree[data.1].0),
            "R" => (data.0 + 1, tree[data.1].1),
            _ => data,
        };

        data = new;

        println!("{:?}", instr);
    }

    println!("{:?}", data);
}
