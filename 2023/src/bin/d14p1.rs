fn main() {
    let input = std::fs::read_to_string("src/input/d14p1.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();

    let ver_lines = (0..grid[0].len())
        .map(|i| join_x(&grid, i))
        .map(|line| shift_rock_line(line))
        .collect::<Vec<_>>();

    for line in &ver_lines {
        println!("{}", line);
    }

    println!("{}", calc_weight(ver_lines));
}

fn join_x(grid: &Vec<Vec<char>>, x: usize) -> String {
    grid.iter()
        .map(|line| line[x].to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn shift_rock_line(line: String) -> String {
    // println!("{:?}", line.split("#").collect::<Vec<_>>());
    line.split("#")
        .map(|s| {
            let mut str = s.split("").collect::<Vec<_>>();
            str.sort_by(|a, b| {
                if *a == "O" {
                    std::cmp::Ordering::Less
                } else if *b == "O" {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            str.join("")
        })
        .collect::<Vec<_>>()
        .join("#")
}

fn calc_weight(lines: Vec<String>) -> usize {
    let len = lines.len();
    lines
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(j, c)| if c == 'O' { len - j } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}
