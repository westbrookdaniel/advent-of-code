fn main() {
    let input = std::fs::read_to_string("src/input/d10p1.txt").unwrap();

    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let line = line.trim();
            line.chars().collect()
        })
        .collect();

    let start = input
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let entries = find_possible_pos(&input, start, dirs);
    let fp = follow_path_tree(&input, start, entries[0]);

    // cleaned grid
    let mut grid = vec![vec!['.'; input[0].len()]; input.len()];
    for (x, y) in fp.path.iter() {
        let prev = input[*y as usize][*x as usize];
        grid[*y as usize][*x as usize] = prev;
    }

    let mut flip_inside = false;
    let mut last_turn = None;
    for (x, y) in fp.path.iter() {
        let lt_dirs = get_lt_dirs_for_char(grid[*y as usize][*x as usize]);
        let rb_dirs = get_rb_dirs_for_char(grid[*y as usize][*x as usize]);

        let c = grid[*y as usize][*x as usize];
        match (last_turn, c) {
            (Some('F'), '7') => {
                last_turn = Some(c);
                flip_inside = !flip_inside
            }
            (Some('L'), 'J') => {
                last_turn = Some(c);
                flip_inside = !flip_inside
            }
            (Some('7'), 'J') => {
                last_turn = Some(c);
                flip_inside = !flip_inside
            }
            (Some('F'), 'L') => {
                last_turn = Some(c);
                flip_inside = !flip_inside
            }
            _ => (),
        };

        let inside_dirs = if flip_inside {
            lt_dirs.clone()
        } else {
            rb_dirs.clone()
        };
        let outside_dirs = if flip_inside {
            rb_dirs.clone()
        } else {
            lt_dirs.clone()
        };

        for d in inside_dirs {
            let inside = (*x as i32 + d.0, *y as i32 + d.1);
            let inside = (inside.0 as usize, inside.1 as usize);
            if inside.0 < grid[0].len() && inside.1 < grid.len() {
                fill_with(&mut grid, inside, 'I');
            }
        }

        for d in outside_dirs {
            let outside = (*x as i32 + d.0, *y as i32 + d.1);
            let outside = (outside.0 as usize, outside.1 as usize);
            if outside.0 < grid[0].len() && outside.1 < grid.len() {
                fill_with(&mut grid, outside, 'O');
            }
        }

        // for clarity
        grid[*y as usize][*x as usize] = ' ';
    }

    let out = grid.iter().fold(0, |acc, line| {
        acc + line.iter().filter(|c| **c == 'I').count()
    });

    println!("{}", out);

    for line in grid.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn find_next_pos(input: &Vec<Vec<char>>, prev: (i32, i32), curr: (i32, i32)) -> Vec<(i32, i32)> {
    let char = input
        .get(curr.1 as usize)
        .and_then(|line| line.get(curr.0 as usize))
        .unwrap();
    let dirs = get_dirs_for_char(*char);
    let possible_pos = find_possible_pos(input, curr, dirs);
    possible_pos
        .iter()
        .filter(|pos| **pos != prev)
        .map(|pos| *pos)
        .collect()
}

fn get_lt_dirs_for_char(char: char) -> Vec<(i32, i32)> {
    match char {
        '|' => vec![(-1, 0)],
        '-' => vec![(0, -1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![],
        '7' => vec![],
        'F' => vec![(-1, 0), (0, -1)],
        _ => vec![],
    }
}

fn get_rb_dirs_for_char(char: char) -> Vec<(i32, i32)> {
    match char {
        '|' => vec![(1, 0)],
        '-' => vec![(0, 1)],
        'L' => vec![],
        'J' => vec![(1, 0), (0, 1)],
        '7' => vec![(1, 0), (0, -1)],
        'F' => vec![],
        _ => vec![],
    }
}

fn get_dirs_for_char(char: char) -> Vec<(i32, i32)> {
    match char {
        '|' => vec![(0, 1), (0, -1)],
        '-' => vec![(1, 0), (-1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        _ => vec![],
    }
}

fn find_possible_pos(
    input: &Vec<Vec<char>>,
    curr: (i32, i32),
    dirs: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    dirs.iter()
        .map(|(dx, dy)| {
            let x = curr.0 + dx;
            let y = curr.1 + dy;

            let char = input
                .get(y as usize)
                .and_then(|line| line.get(x as usize))?;

            match (dx, dy) {
                (0, 1) => match char {
                    '|' => Some((x, y)),
                    'L' => Some((x, y)),
                    'J' => Some((x, y)),
                    'S' => Some((x, y)),
                    _ => None,
                },
                (0, -1) => match char {
                    '|' => Some((x, y)),
                    '7' => Some((x, y)),
                    'F' => Some((x, y)),
                    'S' => Some((x, y)),
                    _ => None,
                },
                (1, 0) => match char {
                    '-' => Some((x, y)),
                    '7' => Some((x, y)),
                    'J' => Some((x, y)),
                    'S' => Some((x, y)),
                    _ => None,
                },
                (-1, 0) => match char {
                    '-' => Some((x, y)),
                    'L' => Some((x, y)),
                    'F' => Some((x, y)),
                    'S' => Some((x, y)),
                    _ => None,
                },
                _ => None,
            }
        })
        .fold(vec![], |mut acc, entry| {
            if let Some(entry) = entry {
                acc.push(entry);
            }
            acc
        })
}

#[derive(Debug)]
struct FollowedPath {
    steps: i32,
    path: Vec<(i32, i32)>,
    next: Option<Vec<(i32, i32)>>,
    contains_s: bool,
}

fn follow_path_tree(input: &Vec<Vec<char>>, prev: (i32, i32), start: (i32, i32)) -> FollowedPath {
    let mut fw = follow_path(input, prev, start);

    if let Some(next) = fw.next.clone() {
        let mut paths = next
            .iter()
            .map(|next| follow_path_tree(input, start, *next))
            .collect::<Vec<_>>();

        paths.sort_by(|a, b| a.steps.cmp(&b.steps));

        fw.steps += paths[0].steps;
        fw.path.append(&mut paths[0].path);
    }

    fw
}

fn follow_path(input: &Vec<Vec<char>>, prev: (i32, i32), start: (i32, i32)) -> FollowedPath {
    let mut fw = FollowedPath {
        steps: 0,
        path: vec![],
        next: None,
        contains_s: false,
    };

    let mut curr = start;
    let mut prev = prev;

    loop {
        let next = find_next_pos(input, prev, curr);

        if next
            .iter()
            .any(|pos| input[pos.1 as usize][pos.0 as usize] == 'S')
        {
            fw.contains_s = true;
        }

        fw.steps += 1;
        fw.path.push(curr);
        prev = curr;

        if next.len() == 0 {
            break;
        }

        if next.len() > 1 {
            fw.next = Some(next);
            break;
        }

        curr = next[0];
    }

    fw
}

fn fill_with(grid: &mut Vec<Vec<char>>, point: (usize, usize), char: char) {
    let mut queue = vec![point];
    let mut i = 0;

    if grid[point.1][point.0] != '.' {
        return;
    }

    grid[point.1][point.0] = char;

    while i < queue.len() {
        let item = queue[i];
        let item = (item.0 as i32, item.1 as i32);
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let next = dirs
            .iter()
            .map(|(dx, dy)| (item.0 + dx, item.1 + dy))
            .collect::<Vec<_>>();
        for pos in next.iter() {
            if pos.1 < 0 || pos.0 < 0 {
                continue;
            }
            if pos.1 >= grid.len() as i32 || pos.0 >= grid[0].len() as i32 {
                continue;
            }

            if grid[pos.1 as usize][pos.0 as usize] == '.' {
                grid[pos.1 as usize][pos.0 as usize] = char;
                let pos = (pos.0 as usize, pos.1 as usize);
                queue.push(pos);
            }
        }
        i += 1;
    }
}
