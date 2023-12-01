fn main() {
    let input = std::fs::read_to_string("src/input/d1p1.txt").unwrap();
    
    // Find first and last number in str (tupal)
    let total = input.lines().map(|line| {
        let mut first = '0';
        let mut last = '0';

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                first = line[i..i+1].parse::<char>().unwrap();
            }
        }

        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                let len = line.len();
                last = line[len-i-1..len-i].parse::<char>().unwrap();
            }
        }

        return format!("{}{}", last, first).parse::<i32>().unwrap();
    }).fold(0, |acc, num| {
        acc + num
    });

    println!("Total: {}", total);
}
