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

    let max = vec![
        (12, "red".to_owned()),
        (13, "green".to_owned()),
        (14, "blue".to_owned()),
    ];

    let total: i32 = input
        .lines()
        .map(|line| Game::from(line))
        .filter(|game| {
            game.subsets.iter().all(|subset| {
                subset.iter().all(|(num, color)| {
                    let set = max.iter().find(|(_, c)| color == c);
                    if let Some(set) = set {
                        return num <= &set.0;
                    }
                    false
                })
            })
        })
        .map(|game| game.id)
        .sum();

    println!("Total: {}", total);
}
