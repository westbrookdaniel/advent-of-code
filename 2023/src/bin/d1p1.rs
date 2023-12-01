fn main() {
    let input = std::fs::read_to_string("src/input/d1p1.txt").unwrap();
    for line in input.lines() {
        println!("{}", line);
    }
}
