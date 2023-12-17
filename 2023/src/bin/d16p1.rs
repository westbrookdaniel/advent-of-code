#[derive(Clone, Hash, Eq, PartialEq)]
struct Beam {
    pos: (isize, isize),
    dir: Dir,
    energised: Vec<Vec<char>>,
    complete: bool,
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

    let mut beams = vec![Beam {
        pos: (0, 0),
        dir: Dir::Right,
        energised: grid.clone(),
        complete: false,
    }];

    while beams.iter().any(|beam| !beam.complete) {
        for i in 0..beams.len() {
            let mut beam = beams[i].clone();

            let char = get_char(&grid, beam.pos);

            if let Some(char) = char {
                if detect_loop(&beam) {
                    beam.complete = true;
                }

                energise(&mut beam);
                let mut next_dirs = match char {
                    '/' => match beam.dir {
                        Dir::Up => vec![Dir::Right],
                        Dir::Down => vec![Dir::Left],
                        Dir::Left => vec![Dir::Down],
                        Dir::Right => vec![Dir::Up],
                    },
                    '\\' => match beam.dir {
                        Dir::Up => vec![Dir::Left],
                        Dir::Down => vec![Dir::Right],
                        Dir::Left => vec![Dir::Up],
                        Dir::Right => vec![Dir::Down],
                    },
                    '|' => match beam.dir {
                        Dir::Up => vec![Dir::Up],
                        Dir::Down => vec![Dir::Down],
                        Dir::Left => vec![Dir::Up, Dir::Down],
                        Dir::Right => vec![Dir::Up, Dir::Down],
                    },
                    '-' => match beam.dir {
                        Dir::Up => vec![Dir::Left, Dir::Right],
                        Dir::Down => vec![Dir::Left, Dir::Right],
                        Dir::Left => vec![Dir::Left],
                        Dir::Right => vec![Dir::Right],
                    },
                    _ => vec![beam.dir],
                };

                let dir = next_dirs.pop().unwrap();

                if next_dirs.len() > 0 {
                    for dir in next_dirs {
                        let mut new_beam = beam.clone();
                        new_beam.dir = dir;
                        new_beam.pos = add(beam.pos, dir);
                        beams.push(new_beam);
                    }
                }

                beam.dir = dir;
                beam.pos = add(beam.pos, dir);
            } else {
                beam.complete = true;
            }

            beams[i] = beam;
        }
    }

    println!("{}", beams.len());

    let combined = beams
        .iter()
        .fold(vec![], |mut acc, beam| {
            if acc.len() == 0 {
                acc = beam.energised.clone();
            } else {
                for (y, row) in beam.energised.iter().enumerate() {
                    for (x, char) in row.iter().enumerate() {
                        match char {
                            '^' | 'v' | '<' | '>' => {
                                acc[y][x] = *char;
                            }
                            _ => {}
                        };
                    }
                }
            }
            acc
        })
        .iter()
        .map(|row| {
            row.iter()
                .map(|char| match char {
                    '^' | 'v' | '<' | '>' => '#',
                    _ => *char,
                })
                .collect::<Vec<char>>()
        })
        // .collect::<Vec<Vec<char>>>();
        .fold(0, |acc, row| {
            acc + row.iter().filter(|char| **char == '#').count()
        });

    // for row in combined {
    //     println!("{}", row.iter().collect::<String>());
    // }

    println!("{}", combined)
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

fn energise(beam: &mut Beam) {
    let pos = (beam.pos.0 as usize, beam.pos.1 as usize);
    beam.energised[pos.1][pos.0] = char_dir(beam.dir);
}

fn char_dir(dir: Dir) -> char {
    match dir {
        Dir::Up => '^',
        Dir::Down => 'v',
        Dir::Left => '<',
        Dir::Right => '>',
    }
}

fn detect_loop(beam: &Beam) -> bool {
    let pos = (beam.pos.0 as usize, beam.pos.1 as usize);
    let dir = char_dir(beam.dir);
    let char = beam.energised[pos.1][pos.0];
    char == dir
}
