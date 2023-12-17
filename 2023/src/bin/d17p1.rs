fn main() {
    let input = std::fs::read_to_string("src/input/d17p1.txt").unwrap();

    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let loss = least_loss_path(&grid, (0, 0), 0, 0, Dir::Right);

    println!("{}", loss);
}

// recursively find the path with the least loss
// you incur loss when you move to a new cell (cell is number of loss
// complete when you reach bottom right corner
// only can go straight or rotate 90 degrees at a time
// once traveling 3 cells in a direction, you must turn
fn least_loss_path(
    grid: &Vec<Vec<char>>,
    pos: (isize, isize),
    loss: usize,
    dist: usize,
    dir: Dir,
) -> usize {
    if pos == (grid[0].len() as isize - 1, grid.len() as isize - 1) {
        return loss;
    }

    let possible_dirs = if dist == 3 {
        match dir {
            Dir::Up => vec![Dir::Left, Dir::Right],
            Dir::Down => vec![Dir::Left, Dir::Right],
            Dir::Left => vec![Dir::Up, Dir::Down],
            Dir::Right => vec![Dir::Up, Dir::Down],
        }
    } else {
        match dir {
            Dir::Up => vec![Dir::Up, Dir::Left, Dir::Right],
            Dir::Down => vec![Dir::Down, Dir::Left, Dir::Right],
            Dir::Left => vec![Dir::Left, Dir::Up, Dir::Down],
            Dir::Right => vec![Dir::Right, Dir::Up, Dir::Down],
        }
    };

    for dir in possible_dirs {
        let mut loss = loss;
        let mut dist = dist;
        let mut pos = pos;

        let char = get_char(&grid, pos);

        if let Some(char) = char {
            loss += char.to_digit(10).unwrap() as usize;
        }

        dist += 1;

        match dir {
            Dir::Up => pos.1 -= 1,
            Dir::Down => pos.1 += 1,
            Dir::Left => pos.0 -= 1,
            Dir::Right => pos.0 += 1,
        }

        least_loss_path(grid, pos, loss, dist, dir);
    }

    0
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn get_char(grid: &Vec<Vec<char>>, pos: (isize, isize)) -> Option<char> {
    let c = grid.get(pos.1 as usize)?.get(pos.0 as usize)?;
    Some(*c)
}
