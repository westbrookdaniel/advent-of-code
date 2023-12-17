use cached::proc_macro::cached;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Beam {
    pos: (isize, isize),
    dir: Dir,
    prev: Vec<Beam>,
}

impl Beam {
    fn from(beam: &Beam, pos: (isize, isize), dir: Dir) -> Beam {
        let mut prev = beam.prev.clone();
        prev.push(beam.clone());
        Beam { pos, dir, prev }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let inputs = std::fs::read_to_string("src/input/d16p1.txt").unwrap();

    let grid = inputs
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut energised = grid.clone();
    move_beams(
        Beam {
            pos: (0, 0),
            dir: Dir::Right,
            prev: vec![],
        },
        &grid,
        &mut energised,
    );

    for line in &energised {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn move_beams(beam: Beam, grid: &Vec<Vec<char>>, energised: &mut Vec<Vec<char>>) {
    let char = get_char(grid, beam.pos);
    for line in energised.clone() {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();

    if let Some(char) = char {
        apply(energised, beam.pos);
        match char {
            '\\' => {
                if check_loop(&beam) {
                    return;
                }

                let dir = match beam.dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
                move_beams(Beam::from(&beam, add(beam.pos, dir), dir), grid, energised);
            }
            '/' => {
                if check_loop(&beam) {
                    return;
                }

                let dir = match beam.dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                };
                move_beams(Beam::from(&beam, add(beam.pos, dir), dir), grid, energised);
            }
            '|' => match beam.dir {
                Dir::Left | Dir::Right => {
                    if check_loop(&beam) {
                        return;
                    }

                    move_beams(
                        Beam::from(&beam, add(beam.pos, Dir::Up), Dir::Up),
                        grid,
                        energised,
                    );
                    move_beams(
                        Beam::from(&beam, add(beam.pos, Dir::Down), Dir::Down),
                        grid,
                        energised,
                    );
                }
                _ => {
                    move_beams(
                        Beam::from(&beam, add(beam.pos, beam.dir), beam.dir),
                        grid,
                        energised,
                    );
                }
            },
            '-' => match beam.dir {
                Dir::Up | Dir::Down => {
                    if check_loop(&beam) {
                        return;
                    }

                    move_beams(
                        Beam::from(&beam, add(beam.pos, Dir::Left), Dir::Left),
                        grid,
                        energised,
                    );
                    move_beams(
                        Beam::from(&beam, add(beam.pos, Dir::Right), Dir::Right),
                        grid,
                        energised,
                    );
                }
                _ => {
                    move_beams(
                        Beam::from(&beam, add(beam.pos, beam.dir), beam.dir),
                        grid,
                        energised,
                    );
                }
            },
            _ => {
                move_beams(
                    Beam::from(&beam, add(beam.pos, beam.dir), beam.dir),
                    grid,
                    energised,
                );
            }
        }
    }
}

fn dir(dir: Dir) -> (isize, isize) {
    match dir {
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
    }
}

fn add(pos: (isize, isize), d: Dir) -> (isize, isize) {
    let d = dir(d);
    let next = (pos.0 + d.0, pos.1 + d.1);
    next
}

fn get_char(grid: &Vec<Vec<char>>, pos: (isize, isize)) -> Option<char> {
    let pos = (pos.0 as usize, pos.1 as usize);
    let c = grid.get(pos.1)?.get(pos.0)?;
    Some(*c)
}

fn apply(grid: &mut Vec<Vec<char>>, pos: (isize, isize)) {
    let pos = (pos.0 as usize, pos.1 as usize);
    grid[pos.1][pos.0] = '#';
}

fn check_loop(beam: &Beam) -> bool {
    // detect if a set of positions repeats
    let mut seen = vec![];
    for pos in beam.prev.iter() {
        if seen.contains(&pos) {
            return true;
        }
        seen.push(pos);
    }
    false
}
