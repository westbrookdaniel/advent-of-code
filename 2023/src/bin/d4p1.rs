#[derive(Debug)]
struct Card {
    nums: Vec<i32>,
    winning_nums: Vec<i32>,
}

impl From<&str> for Card {
    fn from(str: &str) -> Self {
        let (_, nums) = str.split_once(": ").unwrap();
        // let id = id[5..].parse::<i32>().unwrap();
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

        Card { nums, winning_nums }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d4p1.txt").unwrap();

    let out = input
        .lines()
        .map(Card::from)
        .collect::<Vec<Card>>()
        .iter()
        .map(|card| {
            let mut val = 0;
            for num in &card.nums {
                if card.winning_nums.contains(num) {
                    if val == 0 {
                        val = 1;
                    } else {
                        val *= 2
                    }
                }
            }
            val
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("{:?}", out);
}
