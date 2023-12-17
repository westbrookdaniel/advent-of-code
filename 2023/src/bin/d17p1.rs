use pathfinding::directed::dijkstra::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, Dir, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn get_char(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> Option<char> {
    let c = grid.get(pos.1 as usize)?.get(pos.0 as usize)?;
    Some(*c)
}

impl Pos {
    fn loss(&self, grid: &Vec<Vec<char>>) -> u32 {
        let char = get_char(grid, (self.0, self.1));

        if let Some(char) = char {
            char.to_digit(10).unwrap() as u32
        } else {
            10
        }
    }

    fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(Pos, u32)> {
        let &Pos(x, y, dir, dist) = self;

        if dist == 3 {
            let possible_dirs = match dir {
                Dir::Up => vec![Dir::Left, Dir::Right],
                Dir::Down => vec![Dir::Left, Dir::Right],
                Dir::Left => vec![Dir::Up, Dir::Down],
                Dir::Right => vec![Dir::Up, Dir::Down],
            };

            let dist = 0;
            return possible_dirs
                .into_iter()
                .map(|dir| match dir {
                    Dir::Up => Pos(x, y - 1, dir, dist + 1),
                    Dir::Down => Pos(x, y + 1, dir, dist + 1),
                    Dir::Left => Pos(x - 1, y, dir, dist + 1),
                    Dir::Right => Pos(x + 1, y, dir, dist + 1),
                })
                .map(|p| {
                    let l = p.loss(&grid);
                    (p, l)
                })
                .collect();
        } else {
            let possible_dirs = match dir {
                Dir::Up => vec![Dir::Up, Dir::Left, Dir::Right],
                Dir::Down => vec![Dir::Down, Dir::Left, Dir::Right],
                Dir::Left => vec![Dir::Left, Dir::Up, Dir::Down],
                Dir::Right => vec![Dir::Right, Dir::Up, Dir::Down],
            };

            return possible_dirs
                .into_iter()
                .map(|dir| match dir {
                    Dir::Up => Pos(x, y - 1, dir, dist + 1),
                    Dir::Down => Pos(x, y + 1, dir, dist + 1),
                    Dir::Left => Pos(x - 1, y, dir, dist + 1),
                    Dir::Right => Pos(x + 1, y, dir, dist + 1),
                })
                .map(|p| {
                    let l = p.loss(&grid);
                    (p, l)
                })
                .collect();
        };
    }
}

// assert_eq!(result.expect("no path found").1, 4);

fn main() {
    let input = std::fs::read_to_string("src/input/d17p1.txt").unwrap();

    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let goal = (grid[0].len() as i32 - 1, grid.len() as i32 - 1);

    let start_weight = Pos(0, 0, Dir::Right, 0).loss(&grid);

    let result = dijkstra(
        &Pos(0, 0, Dir::Right, 0),
        |p| p.successors(&grid),
        |p| p.0 == goal.0 && p.1 == goal.1,
    )
    .unwrap();

    let l = result.0.iter().fold(0, |acc, p| acc + p.loss(&grid));

    print_grid(&grid, &result);
    println!();
    println!("Result: {}", l - start_weight);
}

fn print_grid(grid: &Vec<Vec<char>>, result: &(Vec<Pos>, u32)) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut found = false;
            for p in &result.0 {
                if p.0 == x as i32 && p.1 == y as i32 {
                    print!("{} ", p.loss(&grid));
                    found = true;
                    break;
                }
            }
            if !found {
                print!("  ");
            }
        }
        println!();
    }
}
