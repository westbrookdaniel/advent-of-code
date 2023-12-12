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

    let perms = unique_perms(&data);

    // for each way unknown_i array can be sorted
    let n = perms
        .par_iter()
        .map(|data| {
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

fn unique_perms(data: &str) -> Vec<String> {
    let mut perms = vec![data.to_string()];
    for i in 0..data.len() {
        let mut new_perms = vec![];
        for mut perm in perms.clone() {
            if perm.chars().nth(i).unwrap() == '?' {
                perm.replace_range(i..i + 1, "#");
                new_perms.push(perm.clone());
                perm.replace_range(i..i + 1, ".");
                new_perms.push(perm.clone());
            } else {
                new_perms.push(perm.clone());
            }
        }
        perms = new_perms;
    }

    perms
}
