use std::collections::hash_map::HashMap;

#[derive(Debug)]
struct Game {
    subsets: Vec<Vec<(i32, String)>>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (_, subsets) = s.split_once(": ").unwrap();
        let subsets = subsets
            .split("; ")
            .map(|subset| {
                subset
                    .split(", ")
                    .map(|item| {
                        let (num, word) = item.split_once(' ').unwrap();
                        let num = num.parse::<i32>().unwrap();
                        let word = word.to_string();
                        (num, word)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Game { subsets }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d2p1.txt").unwrap();

    let total: i32 = input
        .lines()
        .map(|line| Game::from(line))
        .map(|game| {
            let mut maxes = HashMap::from([
                ("red".to_owned(), 0),
                ("green".to_owned(), 0),
                ("blue".to_owned(), 0),
            ]);

            game.subsets.iter().for_each(|subset| {
                subset.iter().for_each(|(num, color)| {
                    let max_num = maxes.get(color).unwrap();
                    maxes.insert(color.to_owned(), *max_num.max(num));
                });
            });

            maxes.values().fold(1, |acc, x| acc * x)
        })
        .sum();

    println!("Total: {}", total);
}
