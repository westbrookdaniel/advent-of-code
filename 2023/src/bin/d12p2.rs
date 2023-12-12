use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input/d12p1.txt").unwrap();

    // let input = input
    //     .lines()
    //     .map(|line| {
    //         let (data, groups) = line.split_at(line.find(" ").unwrap());

    //         let groups = groups
    //             .trim()
    //             .split(",")
    //             .map(|g| g.parse::<i32>().unwrap())
    //             .collect::<Vec<_>>()
    //             .repeat(5);

    //         let data = [data].repeat(5).join("?");

    //         format!(
    //             "{} {}",
    //             data,
    //             groups
    //                 .iter()
    //                 .map(|g| g.to_string())
    //                 .collect::<Vec<_>>()
    //                 .join(",")
    //         )
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");

    let out = input
        .lines()
        .map(|line| find_arrangments(line))
        .collect::<Vec<_>>();
    // .sum::<i32>();

    println!("{:?}", out);
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

    println!();
    println!("{}", data);

    let data = data.chars().collect::<Vec<_>>();

    fn count_valid(data: Vec<char>, groups: Vec<i32>, n: i32) -> i32 {
        if data.len() == 0 {
            let is_perm = groups.len() == 1 && n == groups[0];
            let perfect = groups.len() == 0 && n == 0;
            if is_perm || perfect {
                return 1;
            } else {
                return 0;
            }
        }

        let mut data = data;
        let mut groups = groups;

        return match data.pop().unwrap() {
            '?' => {
                let a_data = [data.clone(), vec!['#']].concat();
                let b_data = [data.clone(), vec!['.']].concat();

                let a = count_valid(a_data, groups.clone(), n);
                let b = count_valid(b_data, groups.clone(), n);

                a + b
            }
            '#' => count_valid(data, groups, n + 1),
            _ => match n {
                0 => count_valid(data, groups, n),
                _ => {
                    if Some(&n) == groups.last() {
                        groups.pop();
                        return count_valid(data, groups, 0);
                    }
                    return 0;
                }
            },
        };
    }

    count_valid(data, groups, 0)
}
