use memoize::memoize;

fn main() {
    let input = std::fs::read_to_string("src/input/d14p1.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();

    let mut grid = grid;
    for _ in 0..1000 {
        grid = cycle(grid);
    }

    println!("{}", calc_weight(grid));
}

#[memoize]
fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    let grid = shift_up(grid);

    let grid = rotate_right(grid);

    grid
}

#[memoize]
fn shift_up(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid = (0..grid[0].len())
        .map(|i| join_x(&grid, i))
        .map(|line| shift_rock_line(line).chars().collect::<Vec<_>>())
        .rev()
        .collect::<Vec<_>>();
    rotate_right(grid)
}

#[memoize]
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

#[memoize]
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

#[memoize]
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
