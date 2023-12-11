// | is a vertical pipe connecting north and south
// - is a horizontal pipe connecting east and west
// L is a 90-degree bend connecting north and east
// J is a 90-degree bend connecting north and west
// 7 is a 90-degree bend connecting south and west
// F is a 90-degree bend connecting south and east
// . is ground; there is no pipe in this tile

fn main() {
    let input = std::fs::read_to_string("src/input/d10p1.txt").unwrap();

    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

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

    // let paths = entries
    //     .iter()
    //     .map(|entry| follow_path_tree(&input, start, *entry))
    //     .collect::<Vec<_>>();

    let path = follow_path_tree(&input, start, entries[0]);

    let dist = (path.steps as f32 / 2.0).ceil() as i32;

    println!("{:?}", dist);
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
                    _ => None,
                },
                (0, -1) => match char {
                    '|' => Some((x, y)),
                    '7' => Some((x, y)),
                    'F' => Some((x, y)),
                    _ => None,
                },
                (1, 0) => match char {
                    '-' => Some((x, y)),
                    '7' => Some((x, y)),
                    'J' => Some((x, y)),
                    _ => None,
                },
                (-1, 0) => match char {
                    '-' => Some((x, y)),
                    'L' => Some((x, y)),
                    'F' => Some((x, y)),
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
    };

    let mut curr = start;
    let mut prev = prev;

    loop {
        let next = find_next_pos(input, prev, curr);

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
