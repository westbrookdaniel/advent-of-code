fn main() {
    let input = std::fs::read_to_string("src/input/d13p1.txt").unwrap();

    let chunks = input.split("\r\r\n\r\r").collect::<Vec<_>>();

    let grids = chunks
        .iter()
        .map(|group| {
            group
                .lines()
                .map(|line| line.trim().chars().collect::<Vec<_>>())
                .filter(|line| line.len() > 0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = grids.iter().map(|grid| find_mirror_val(grid)).sum::<i32>();

    println!("{}", out);
}

fn find_mirror_val(grid: &Vec<Vec<char>>) -> i32 {
    let ver_lines = (0..grid[0].len())
        .map(|i| join_x(&grid, i))
        .collect::<Vec<_>>();

    let hor_lines = grid
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>();

    let v = lines_left_of_mirror(ver_lines, vec![]);
    let h = lines_left_of_mirror(hor_lines, vec![]);

    v + (100 * h)
}

fn lines_left_of_mirror(left: Vec<String>, right: Vec<String>) -> i32 {
    let mut left = left;

    if left.len() == 0 {
        return 0;
    }

    if right.len() == 0 || !is_equal(&left, &right) {
        let c = left.pop().unwrap();
        return lines_left_of_mirror(left, [vec![c], right].concat());
    }

    return left.len() as i32;
}

fn join_x(grid: &Vec<Vec<char>>, x: usize) -> String {
    grid.iter()
        .map(|line| line[x].to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn is_equal(left: &Vec<String>, right: &Vec<String>) -> bool {
    left.iter().rev().zip(right.iter()).all(|(l, r)| l == r)
}
