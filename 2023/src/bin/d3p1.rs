fn main() {
    let input = std::fs::read_to_string("src/input/d3p1.txt").unwrap();

    let vectors = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut symbols = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, symbol) in line.iter().enumerate() {
            if is_symbol(*symbol) {
                symbols.push((i as i32, j as i32));
            }
        }
    }

    let mut numbers_around_symbols = vec![];

    for (i, j) in symbols.iter() {
        for (vi, vj) in vectors {
            let i = i + vi;
            let j = j + vj;

            if i < 0 || j < 0 {
                continue;
            }
            if i >= grid.len() as i32 || j >= grid[i as usize].len() as i32 {
                continue;
            }

            let num = cut_number(&mut grid, &i, &j);
            if let Some(num) = num {
                numbers_around_symbols.push(num);
            }
        }
    }

    println!("{:?}", numbers_around_symbols.iter().sum::<i32>());
}

fn is_symbol(c: char) -> bool {
    if c == '.' {
        return false;
    }
    !c.is_numeric() && !c.is_alphabetic()
}

fn is_not_number(c: char) -> bool {
    c == ' ' || c == '\n' || c == '.' || is_symbol(c)
}

fn cut_number(grid: &mut [Vec<char>], i: &i32, j: &i32) -> Option<i32> {
    let start = grid[*i as usize][*j as usize];

    if is_not_number(start) {
        return None;
    }

    // Find where number starts that contains point (i, j)
    // and cut it out
    let line = &mut grid[*i as usize];
    let number = {
        let mut is_num = false;
        let mut current_num = String::new();
        for (k, c) in line.iter().enumerate() {
            if k as i32 == *j {
                is_num = true;
            }
            if is_not_number(*c) && is_num {
                // Replace the full number in grid with .
                line[k - current_num.len()..k]
                    .iter_mut()
                    .for_each(|c| *c = '.');
                break;
            }
            if is_not_number(*c) {
                current_num.clear();
            } else {
                current_num.push(*c);
            }
        }
        current_num.parse::<i32>().unwrap()
    };

    Some(number)
}
