fn main() {
    let input = std::fs::read_to_string("src/input/d12p1.txt").unwrap();

    let input = input
        .lines()
        .map(|line| {
            let (data, groups) = line.split_at(line.find(" ").unwrap());

            let groups = groups
                .trim()
                .split(",")
                .map(|g| g.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);

            let data = [data].repeat(5).join("?");

            format!(
                "{} {}",
                data,
                groups
                    .iter()
                    .map(|g| g.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let out = input
        .lines()
        .map(|line| find_arrangments(line))
        .collect::<Vec<_>>();
    // .sum::<i32>();

    // println!("{:?}", out);
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

    let d = data.len();
    let g = groups.len();
    let mut dp = vec![vec![0; g + 1]; d + 1];

    for char in (0..d).rev() {
        for group in (0..g).rev() {
            if data.chars().nth(char).unwrap() == '?' {
                dp[char][group] = dp[char + 1][group] + dp[char + 1][group + 1];
            } else {
                dp[char][group] = dp[char + 1][group];
            }
        }
    }

    println!("{:?}", groups);
    for d in &dp {
        println!("{:?}", d);
    }

    dp[0][0]
}
