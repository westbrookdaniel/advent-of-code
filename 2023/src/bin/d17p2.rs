use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, Dir, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Pos {
    fn loss(&self, grid: &Matrix<u32>) -> u32 {
        let dig = grid.get((self.0 as usize, self.1 as usize)).unwrap();
        *dig
    }

    fn successors(&self, grid: &Matrix<u32>) -> Vec<(Pos, u32)> {
        let &Pos(x, y, curr_dir, mut dist) = self;

        let mut possible_dirs = vec![];

        if dist > 3 {
            possible_dirs = match curr_dir {
                Dir::Up => vec![Dir::Left, Dir::Right],
                Dir::Down => vec![Dir::Left, Dir::Right],
                Dir::Left => vec![Dir::Up, Dir::Down],
                Dir::Right => vec![Dir::Up, Dir::Down],
            };
        }

        if dist < 10 {
            possible_dirs.push(curr_dir);
        } else {
            dist = 0;
        }

        let dirs = possible_dirs
            .into_iter()
            .map(|dir| {
                let mut dist = dist;
                if dir != curr_dir {
                    dist = 0;
                }
                match dir {
                    Dir::Up => Pos(x, y - 1, dir, dist + 1),
                    Dir::Down => Pos(x, y + 1, dir, dist + 1),
                    Dir::Left => Pos(x - 1, y, dir, dist + 1),
                    Dir::Right => Pos(x + 1, y, dir, dist + 1),
                }
            })
            .filter(|p| is_oob(p, grid))
            .map(|p| {
                let l = p.loss(&grid);
                (p, l)
            })
            .collect::<Vec<_>>();

        dirs
    }
}

fn is_oob(pos: &Pos, grid: &Matrix<u32>) -> bool {
    let &Pos(x, y, _, _) = pos;
    x >= 0 && y >= 0 && x < grid.rows as i32 && y < grid.columns as i32
}

fn main() {
    let input = std::fs::read_to_string("src/input/d17p1.txt").unwrap();

    let grid = Matrix::from_rows(input.trim().lines().map(|line| {
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    }))
    .unwrap();

    let goal = (grid.rows as i32 - 1, grid.columns as i32 - 1);

    let result = dijkstra(
        &Pos(0, 0, Dir::Down, 0),
        |p| p.successors(&grid),
        |p| p.0 == goal.0 && p.1 == goal.1 && p.3 > 3,
    )
    .unwrap();

    print_journey(&result.0, &grid);

    let init = result.0[0].loss(&grid);
    let l = result.0.iter().fold(0, |acc, p| acc + p.loss(&grid));

    println!("{}", l - init);
}

fn print_journey(pos: &Vec<Pos>, grid: &Matrix<u32>) {
    for x in 0..grid.rows {
        for y in 0..grid.columns {
            let mut found = false;
            for p in pos {
                if p.0 == x as i32 && p.1 == y as i32 {
                    found = true;
                    break;
                }
            }

            if found {
                print!("{}", grid.get((x, y)).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
