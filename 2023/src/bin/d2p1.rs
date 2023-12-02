use std::collections::hash_map::HashMap;

#[derive(Debug)]
struct Game {
    id: i32,
    subsets: Vec<Vec<(i32, String)>>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (id, subsets) = s.split_once(": ").unwrap();
        let id = id[5..].parse::<i32>().unwrap();
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

        Game { id, subsets }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d2p1.txt").unwrap();

    let maxes = HashMap::from([
        ("red".to_owned(), 12),
        ("green".to_owned(), 13),
        ("blue".to_owned(), 14),
    ]);

    let total: i32 = input
        .lines()
        .map(|line| Game::from(line))
        .filter(|game| {
            game.subsets.iter().all(|subset| {
                subset.iter().all(|(num, color)| {
                    let max_num = maxes.get(color).unwrap();
                    num <= max_num
                })
            })
        })
        .map(|game| game.id)
        .sum();

    println!("Total: {}", total);
}
