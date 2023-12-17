use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Beam {
    pos: (isize, isize),
    dir: Dir,
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

    let mut energised: HashMap<(isize, isize), Vec<Dir>> = HashMap::new();

    let mut beams = vec![Beam {
        pos: (0, 0),
        dir: Dir::Right,
        complete: false,
    }];

    while beams.iter().any(|beam| !beam.complete) {
        for i in 0..beams.len() {
            let mut beam = beams[i].clone();

            if beam.complete {
                continue;
            }

            let char = get_char(&grid, beam.pos);

            if let Some(char) = char {
                if detect_loop(&beam, &energised) {
                    beam.complete = true;
                }

                energise(&beam, &mut energised);
                let mut next_dirs = match char {
                    '/' => match beam.dir {
                        Dir::Up => {
                            vec![Dir::Right]
                        }
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

    println!("{}", energised.len());
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

fn energise(beam: &Beam, energised: &mut HashMap<(isize, isize), Vec<Dir>>) {
    if let Some(_) = energised.get(&beam.pos) {
        let dirs = energised.get_mut(&beam.pos).unwrap();
        if !dirs.contains(&beam.dir) {
            dirs.push(beam.dir);
        }
    } else {
        energised.insert(beam.pos, vec![beam.dir]);
    }
}

fn detect_loop(beam: &Beam, energised: &HashMap<(isize, isize), Vec<Dir>>) -> bool {
    if let Some(dirs) = energised.get(&beam.pos) {
        dirs.contains(&beam.dir)
    } else {
        false
    }
}
