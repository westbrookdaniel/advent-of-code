fn main() {
    let input = std::fs::read_to_string("src/input/d1p1.txt").unwrap();

    let total = input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            // First
            for (i, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    first = c.to_digit(10).unwrap();
                    break;
                } else if c.is_alphabetic() {
                    let num = is_word_num(i, line, true);
                    if let Some(n) = num {
                        first = n;
                        break;
                    }
                }
            }

            // Last
            for (i, c) in line.chars().rev().enumerate() {
                if c.is_numeric() {
                    last = c.to_digit(10).unwrap();
                    break;
                } else if c.is_alphabetic() {
                    let num = is_word_num(line.len() - i, line, false);
                    if let Some(n) = num {
                        last = n;
                        break;
                    }
                }
            }

            return format!("{}{}", first, last).parse::<u32>().unwrap();
        })
        .fold(0, |acc, num| {
            println!("{}", num);
            acc + num
        });

    println!("Total: {}", total);
}

fn is_word_num(i: usize, line: &str, forward: bool) -> Option<u32> {
    let nums_word = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    if forward {
        for (j, word) in nums_word.iter().enumerate() {
            let is_oob = i + word.len() > line.len();
            if !is_oob && line[i..i + word.len()].to_lowercase() == *word {
                return Some(nums[j]);
            }
        }
    } else {
        for (j, word) in nums_word.iter().enumerate() {
            let is_oob = i < word.len();
            if !is_oob && line[i - word.len()..i].to_lowercase() == *word {
                return Some(nums[j]);
            }
        }
    }

    return None;
}
