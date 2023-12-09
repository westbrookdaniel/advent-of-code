fn main() {
    let input = std::fs::read_to_string("src/input/d9p1.txt").unwrap();

    let out = input
        .lines()
        .map(|line| {
            let seq = line
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            // seq.reverse();

            let mut seq = vec![seq];

            while !is_break_time(&seq) {
                let layers = seq.len();

                for _ in 0..layers {
                    if is_break_time(&seq) {
                        break;
                    }

                    for j in 0..=seq.len() - 1 {
                        let k = seq.len() - j - 1;

                        if is_break_time(&seq) {
                            break;
                        }

                        let n = seq[j][k] - seq[j][k + 1];

                        if j < seq.len() - 1 {
                            seq[j + 1].push(n);
                        } else {
                            seq.push(vec![n]);
                        }
                    }
                }
            }

            let mut i = seq.len() - 2;
            let mut n = seq[i][0];
            while i > 0 {
                n = n + seq[i - 1][0];
                i -= 1;
            }
            n
        })
        .sum::<i32>();

    println!("{:?}", out);
}

fn is_break_time(seq: &Vec<Vec<i32>>) -> bool {
    let second_last_is_zero = seq.len() > 1 && seq[seq.len() - 2][1] == 0;
    let last_is_zero = seq[seq.len() - 1][0] == 0;
    second_last_is_zero && last_is_zero
}
