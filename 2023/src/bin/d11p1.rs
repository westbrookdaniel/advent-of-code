use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input/d11p1.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| {
            let line = line.trim();
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // Expand rows
    let expanded = grid
        .iter()
        .map(|row| {
            if row.iter().all(|c| *c == '.') {
                return vec![row.clone(), row.clone()];
            }
            vec![row.clone()]
        })
        .flatten()
        .collect::<Vec<Vec<char>>>();

    // Expand columns
    let mut expanded = expanded.clone();
    let mut i = 0;
    let mut max = expanded[0].len() - 1;
    while i < max {
        let mut col = vec![];
        for row in &expanded {
            col.push(row[i]);
        }
        if col.iter().all(|c| *c == '.') {
            for row in &mut expanded {
                row.insert(i, '.');
            }
            i += 1;
            max += 1;
        }
        i += 1;
    }
    let expanded = expanded;

    let mut id = 1;
    // id, (x, y)
    let mut galaxies = HashMap::new();
    expanded.iter().enumerate().for_each(|(i, row)| {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.insert(id, (j, i));
                id += 1;
            }
        }
    });

    let pairs = unique_pairs(id - 1);

    let out = pairs
        .iter()
        .map(|pair| {
            let galaxy1 = galaxies.get(&pair.0).unwrap();
            let galaxy2 = galaxies.get(&pair.1).unwrap();
            let dist = shortest_distance(galaxy1, galaxy2);
            dist
        })
        .sum::<usize>();

    println!("{}", out);
}

fn unique_pairs(n: usize) -> Vec<(usize, usize)> {
    let mut pairs = vec![];
    for i in 1..=n {
        for j in i + 1..=n {
            pairs.push((i, j));
        }
    }
    pairs
}

fn shortest_distance(galaxy1: &(usize, usize), galaxy2: &(usize, usize)) -> usize {
    let x1 = galaxy1.0 as isize;
    let y1 = galaxy1.1 as isize;
    let x2 = galaxy2.0 as isize;
    let y2 = galaxy2.1 as isize;

    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();

    let dist = dx + dy;

    dist as usize
}
