fn main() {
    let input = std::fs::read_to_string("src/input/d1p1.txt").unwrap();

    let nums_word = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let total: u32 = input
        .lines()
        .map(|line| {
            let mut matches = vec![];

            for (i, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    matches.push(c.to_digit(10).unwrap());
                }
                for (j, word) in nums_word.iter().enumerate() {
                    if line[i..].starts_with(word) {
                        matches.push((j + 1) as u32);
                    }
                }
            }

            let first = matches.first().unwrap();
            let last = matches.last().unwrap();

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum();

    println!("Total: {}", total);
}
