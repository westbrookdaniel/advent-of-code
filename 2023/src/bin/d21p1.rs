use std::collections::VecDeque;

use pathfinding::matrix::Matrix;

fn main() {
    let mut input = std::fs::read_to_string("src/input/d21p1.txt").unwrap();

    // position where char is S
    let start = {
        let mut x = 0;
        let mut y = 0;
        for (i, line) in input.trim().lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                if c == 'S' {
                    x = i;
                    y = j;
                    break;
                }
            }
        }
        (x, y)
    };

    input = input.replace('S', &".");

    let grid = Matrix::from_rows(
        input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>()),
    )
    .unwrap();

    let mut points = VecDeque::new();
    points.push_back(start);

    for _ in 0..64 {
        let mut queue = points.clone();
        points.clear();

        while queue.len() > 0 {
            let point = queue.pop_front().unwrap();
            let possible = possible_moves(point);

            let possible = possible
                .iter()
                .filter(|p| {
                    let char = grid.get(**p);
                    if let Some(char) = char {
                        return char == &'.';
                    }
                    false
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

    // // put queue points onto grid as O
    // for (i, line) in input.trim().lines().enumerate() {
    //     for (j, char) in line.trim().chars().enumerate() {
    //         let found = points.iter().find(|p| **p == (i, j));
    //         if found.is_some() {
    //             print!("O");
    //         } else {
    //             print!("{}", char);
    //         }
    //     }
    //     println!();
    // }

    println!("{}", points.len())
}

fn possible_moves(point: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = point;
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}
