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

    let out = grids
        .iter()
        .map(|grid| find_smudged_mirror_val(grid))
        .sum::<i32>();

    println!("{}", out);
}

fn find_smudged_mirror_val(grid: &Vec<Vec<char>>) -> i32 {
    let (val1, val2) = find_mirror_vals(grid);

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let mut smudged = grid.clone();
            smudged[y][x] = smudge(*c);

            let (val3, val4) = find_mirror_vals(&smudged);

            if is_diff_val(val1, val3) && !empty_val(val3) {
                return diff_val(val3, val1);
            }

            if is_diff_val(val2, val4) && !empty_val(val4) {
                return diff_val(val4, val2);
            }
        }
    }

    panic!("No new mirror found");
}

fn empty_val(val: (i32, i32)) -> bool {
    val.0 == 0 && val.1 == 0
}

fn is_diff_val(val1: (i32, i32), val2: (i32, i32)) -> bool {
    val1.0 != val2.0 || val1.1 != val2.1
}

fn diff_val(val: (i32, i32), prev_val: (i32, i32)) -> i32 {
    let mut c = 0;
    if val.0 != prev_val.0 {
        c += val.0;
    }
    if val.1 != prev_val.1 {
        c += val.1 * 100;
    }
    c
}

fn smudge(c: char) -> char {
    match c {
        '#' => '.',
        '.' => '#',
        _ => c,
    }
}

fn find_mirror_vals(grid: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    let mut ver_lines = (0..grid[0].len())
        .map(|i| join_x(&grid, i))
        .collect::<Vec<_>>();

    let mut hor_lines = grid
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>();

    let v_len = ver_lines.len() as i32;
    let h_len = hor_lines.len() as i32;

    let v = lines_left_of_mirror(ver_lines.clone(), vec![]);
    let h = lines_left_of_mirror(hor_lines.clone(), vec![]);

    ver_lines.reverse();
    hor_lines.reverse();

    let mut v2 = lines_left_of_mirror(ver_lines, vec![]);
    let mut h2 = lines_left_of_mirror(hor_lines, vec![]);

    if v2 != 0 {
        v2 = v_len - v2;
    }
    if h2 != 0 {
        h2 = h_len - h2;
    }

    ((v, h), (v2, h2))
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
