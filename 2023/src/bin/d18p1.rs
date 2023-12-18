fn main() {
    let input = std::fs::read_to_string("src/input/d18p1.txt").unwrap();

    let steps = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim().split(' ').collect::<Vec<&str>>();

            let dir = match line[0] {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!("Invalid direction"),
            };

            let n = line[1].parse::<usize>().unwrap();

            (dir, n)
        })
        .collect::<Vec<(Dir, usize)>>();

    // Walk steps to build points
    let mut points = vec![];
    let mut x = 0;
    let mut y = 0;
    for step in steps {
        let d = pos_from_dir(step.0);
        for _ in 0..step.1 {
            x += d.0;
            y += d.1;
            points.push((x, y));
        }
    }

    // Convert to grid
    let largest_x = points.iter().map(|p| p.0).max().unwrap();
    let smallest_x = points.iter().map(|p| p.0).min().unwrap();
    let largest_y = points.iter().map(|p| p.1).max().unwrap();
    let smallest_y = points.iter().map(|p| p.1).min().unwrap();

    let mut grid = vec![
        vec!['.'; (largest_x - smallest_x + 1) as usize];
        (largest_y - smallest_y + 1) as usize
    ];

    for point in points {
        grid[(point.1 - smallest_y) as usize][(point.0 - smallest_x) as usize] = '#';
    }

    // flood fill inside
    let start = {
        let y = 1;
        let mut s = None;
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] == '#' {
                s = Some((x + 1, y));
                break;
            }
        }
        s.unwrap()
    };
    flood_fill(&mut grid, start);

    for line in &grid {
        println!("{}", line.iter().collect::<String>());
    }
    println!();

    let out = grid.iter().flatten().filter(|c| **c == '#').count();
    println!("{}", out);
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

fn flood_fill(grid: &mut Vec<Vec<char>>, start: (usize, usize)) {
    let mut stack = vec![start];
    while let Some((x, y)) = stack.pop() {
        if grid[y][x] == '.' {
            grid[y][x] = '#';
            stack.push((x + 1, y));
            stack.push((x - 1, y));
            stack.push((x, y + 1));
            stack.push((x, y - 1));
        }
    }
}
