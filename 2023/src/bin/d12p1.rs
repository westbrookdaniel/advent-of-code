use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("src/input/d12p1.txt").unwrap();

    let out = input
        .lines()
        .par_bridge()
        .map(|line| find_arrangments(line))
        .sum::<i32>();

    println!("{}", out);
}

// Find ways to fill ? with # that satisfy numbers
// .??..??...?##. 1,1,3
fn find_arrangments(line: &str) -> i32 {
    let (data, groups) = line.split_at(line.find(" ").unwrap());
    let groups = groups
        .trim()
        .split(",")
        .map(|g| g.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let unknown_i = data
        .trim()
        .split("")
        .enumerate()
        .filter(|(_, c)| c == &"?")
        .map(|(i, _)| i - 1)
        .collect::<Vec<_>>();

    let total = groups.iter().sum::<i32>();
    let total_hash = data.trim().matches("#").count() as i32;
    let hash_to_add = total - total_hash;

    let perms = unknown_i
        .iter()
        .enumerate()
        .map(|(i, _)| if hash_to_add > i as i32 { "#" } else { "." })
        .permutations(unknown_i.len())
        .unique()
        .collect::<Vec<_>>();

    // for each way unknown_i array can be sorted
    let n = perms
        .par_iter()
        .map(|perm| {
            let mut data = data.to_string();
            for (i, p) in perm.iter().enumerate() {
                // replace at index in str
                data.replace_range(unknown_i[i]..unknown_i[i] + 1, p);
            }

            if !validate(&data, &groups) {
                return 0;
            }
            return 1;
        })
        .sum::<i32>();

    n
}

fn validate(data: &str, groups: &[i32]) -> bool {
    let data_groups = data
        .trim()
        .split(".")
        .filter(|g| !g.is_empty())
        .map(|g| g.len() as i32)
        .collect::<Vec<_>>();

    data_groups == *groups
}
