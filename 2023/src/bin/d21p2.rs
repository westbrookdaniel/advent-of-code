use std::collections::VecDeque;

use cached::proc_macro::cached;
use pathfinding::matrix::Matrix;

fn main() {
    let input = std::fs::read_to_string("src/input/d21p1.txt").unwrap();

    // position where char is S
    let start = {
        let mut x = 0;
        let mut y = 0;
        for (i, line) in input.trim().lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                if c == 'S' {
                    x = i as isize;
                    y = j as isize;
                    break;
                }
            }
        }
        (x, y)
    };

    let grid = input.replace('S', &".");

    let mut points = VecDeque::new();
    points.push_back(start);

    let lines = grid.trim().lines().collect::<Vec<_>>();
    let columns = lines[0].trim().chars().count();
    let rows = lines.len();

    for i in 0..100 {
        println!("{}", i);

        let mut queue = points.clone();
        points.clear();

        while queue.len() > 0 {
            let point = queue.pop_front().unwrap();
            let possible = possible_moves(point);

            let possible = possible
                .iter()
                .filter(|p| {
                    let p = **p;
                    let point = wrap_point(p, rows, columns);
                    let char = get_char(grid.clone(), point).unwrap();
                    char == '.'
                })
                .collect::<Vec<_>>();

            points.extend(possible);
        }

        // filter repeated
        points = points
            .iter()
            .map(|p| *p)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<VecDeque<_>>();
    }

    println!("{}", points.len());
}

fn possible_moves(point: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = point;
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}


fn wrap_point(point: (isize, isize), rows: usize, columns: usize) -> (usize, usize) {
    let mut point = (point.0 % rows as isize, point.1 % columns as isize);
    if point.0 < 0 {
        point.0 = rows as isize + point.0
    }
    if point.1 < 0 {
        point.1 = columns as isize + point.1
    }
    (point.0 as usize, point.1 as usize)
}

#[cached]
fn get_char(grid: String, point: (usize, usize)) -> Option<char> {
    let lines = grid.trim().lines().collect::<Vec<_>>();
    let columns = lines[0].trim().chars().count();
    let rows = lines.len();

    let (x, y) = point;
    if x >= rows || y >= columns {
        return None;
    }

    let line = lines[x];
    let char = line.chars().nth(y)?;
    Some(char)
}
