
fn main() {
    let input = std::fs::read_to_string("src/input/d1p1.txt").unwrap();
    
    // Find first and last number in str (tupal)
    let total = input.lines().map(|line| {
        let mut first = '0';
        let mut last = '0';

        // First
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                first = line[i..i+1].parse::<char>().unwrap();
                break;
            } else if c.is_alphabetic() {
                let num = is_word_num(i, line, true);
                if num.is_some() {
                    first = num.unwrap().to_string().parse::<char>().unwrap();
                    break;
                }
            }
        }

        // Last
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                let len = line.len();
                last = line[len-i-1..len-i].parse::<char>().unwrap();
                break;
            } else if c.is_alphabetic() {
                let num = is_word_num(line.len()-i, line, false);
                if num.is_some() {
                    last = num.unwrap().to_string().parse::<char>().unwrap();
                    break;
                }
            }
        }

        return format!("{}{}", first, last).parse::<i32>().unwrap();
    }).fold(0, |acc, num| {
        println!("{}", num);
        acc + num
    });

    println!("Total: {}", total);
}

fn is_word_num(i: usize, line: &str, forward: bool) -> Option<i32> {
    let nums_word = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    if forward {
        for (j, word) in nums_word.iter().enumerate() {
            let is_oob = i+word.len() > line.len();
            if !is_oob && line[i..i+word.len()].to_lowercase() == *word {
                return Some(nums[j]);
            }
        }
    } else {
        for (j, word) in nums_word.iter().enumerate() {
            let is_oob = i < word.len();
            if !is_oob && line[i-word.len()..i].to_lowercase() == *word {
                return Some(nums[j]);
            }
        }
    }

    return None
}
