use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("src/input/d18p1.txt").unwrap();

    let steps = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim().split(' ').collect::<Vec<&str>>();
            let line = line[2];
            let dir = line.chars().nth(line.len() - 2).unwrap();
            let n = line[2..line.len() - 2].to_string();
            let n = usize::from_str_radix(&n, 16).unwrap();
            let dir = match dir {
                '3' => Dir::Up,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '0' => Dir::Right,
                _ => panic!("Invalid direction"),
            };
            (dir, n)
        })
        // .map(|line| {
        //     let line = line.trim().split(' ').collect::<Vec<&str>>();
        //     let dir = match line[0] {
        //         "U" => Dir::Up,
        //         "D" => Dir::Down,
        //         "L" => Dir::Left,
        //         "R" => Dir::Right,
        //         _ => panic!("Invalid direction"),
        //     };
        //     let n = line[1].parse::<usize>().unwrap();
        //     (dir, n)
        // })
        .collect::<Vec<(Dir, usize)>>();

    // Walk steps to build points
    let mut points: Vec<(i32, i32)> = vec![];
    let mut total_dist = 0;
    let mut x = 0;
    let mut y = 0;
    for step in steps {
        let d = pos_from_dir(step.0);
        x += d.0 * step.1 as i32;
        y += d.1 * step.1 as i32;
        points.push((x, y));
        total_dist += step.1;
    }

    // shoelace formula to get the area of polygon
    let area = points
        .par_iter()
        .enumerate()
        .map(|(i, p)| {
            let j = (i + 1) % points.len();
            p.0 as i64 * points[j].1 as i64 - p.1 as i64 * points[j].0 as i64
        })
        .sum::<i64>()
        .abs()
        / 2;

    // pick's therom to get full area
    let area = area as f64 + 0.5 * total_dist as f64 - 1.0;

    println!("{}", area + 2.0);
}

fn pos_from_dir(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Up => (0, 1),
        Dir::Down => (0, -1),
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
