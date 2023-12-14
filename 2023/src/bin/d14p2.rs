use cached::proc_macro::cached;

fn main() {
    let str_grid = std::fs::read_to_string("src/input/d14p1.txt").unwrap();

    let mut str_grid = str_grid;
    for _ in 0..1000 {
        str_grid = cycle(str_grid);
    }

    let grid = str_grid
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();

    println!("{}", calc_weight(grid));
}

#[cached]
fn cycle(str_grid: String) -> String {
    let grid = str_grid
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    grid.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn shift_up(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = (0..grid[0].len())
        .map(|i| join_x(&grid, i))
        .map(|line| shift_rock_line(line).chars().collect::<Vec<_>>())
        .rev()
        .collect::<Vec<_>>();
    rotate_right(grid)
}

fn rotate_right(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            new_grid[j][grid.len() - i - 1] = grid[i][j];
        }
    }
    new_grid
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

fn calc_weight(grid: Vec<Vec<char>>) -> usize {
    let lines = (0..grid[0].len())
        .map(|i| join_x(&grid, i))
        .collect::<Vec<_>>();

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
