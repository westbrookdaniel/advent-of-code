#[derive(Debug, Clone)]
struct Card {
    id: i32,
    nums: Vec<i32>,
    winning_nums: Vec<i32>,
}

impl From<&str> for Card {
    fn from(str: &str) -> Self {
        let (id, nums) = str.split_once(": ").unwrap();
        let id = id[4..].trim().parse::<i32>().unwrap();
        let (winning_nums, nums) = nums.split_once(" | ").unwrap();
        let nums = nums
            .split(' ')
            .filter(|n| n.trim() != "")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let winning_nums = winning_nums
            .split(' ')
            .filter(|n| n.trim() != "")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        Card {
            id,
            nums,
            winning_nums,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d4p1.txt").unwrap();

    let cards = input.lines().map(Card::from).collect::<Vec<Card>>();
    let mut copies: Vec<i32> = Vec::new();

    cards.iter().for_each(|card| {
        let mut wins = 0;
        for num in &card.nums {
            if card.winning_nums.contains(num) {
                wins += 1;
            }
        }

        let to_copy = card.id..(wins + card.id);
        let n = copies.iter().filter(|id| **id == card.id).count() + 1;

        copies.extend(to_copy.flat_map(|id| (1..=n).map(move |_| id + 1)));
    });

    println!("{}", cards.len() + copies.len());
}
