fn main() {
    let input = std::fs::read_to_string("src/input/d9p1.txt").unwrap();

    let out = input
        .lines()
        .map(|line| {
            let mut seq = line
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            seq.reverse();

            let mut seq = vec![seq];

            while !second_last_is_zero(&seq) {
                let layers = seq.len();

                for _ in 0..layers {
                    if second_last_is_zero(&seq) {
                        break;
                    }

                    for j in 0..=seq.len() - 1 {
                        let k = seq.len() - j - 1;

                        if second_last_is_zero(&seq) {
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

fn second_last_is_zero(seq: &Vec<Vec<i32>>) -> bool {
    seq.len() > 1 && seq[seq.len() - 2][1] == 0
}
