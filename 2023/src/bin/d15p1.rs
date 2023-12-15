fn main() {
    let inputs = std::fs::read_to_string("src/input/d15p1.txt").unwrap();

    let out = inputs
        .split(',')
        .map(|input| {
            let mut v: i32 = 0;
            for char in input.trim().chars() {
                v += char as i32;
                v = v * 17;
                v = v % 256;
            }
            v
        })
        .sum::<i32>();

    println!("{}", out);
}
