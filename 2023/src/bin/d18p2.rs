use cached::proc_macro::cached;
use linya::{Bar, Progress};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

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
            println!("{} {}", dir, n);
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
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for step in steps {
        let d = pos_from_dir(step.0);
        for _ in 0..step.1 {
            x += d.0;
            y += d.1;
            rows.entry(y).or_insert(vec![]).push(x);
        }
    }

    let values = rows.values().flatten().collect::<Vec<&i32>>();
    let largest_x = *values.iter().max().unwrap();
    let smallest_x = *values.iter().min().unwrap();
    let width = (largest_x.clone() - smallest_x.clone() + 1) as usize;

    let progress = Mutex::new(Progress::new());
    let bar: Bar = progress.lock().unwrap().bar(rows.len(), format!("Loading"));

    // find all the inside points
    let out = rows
        .par_iter()
        .map(|(_, row)| {
            let n = calc_row(row.clone(), smallest_x.clone(), width);

            progress.lock().unwrap().inc_and_draw(&bar, 1);

            n
        })
        .sum::<usize>();

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

#[cached]
fn calc_row(row: Vec<i32>, smallest_x: i32, width: usize) -> usize {
    let mut str_row = String::from_utf8(vec![' ' as u8; width]).unwrap();
    for x in row {
        let x = x - smallest_x;
        str_row.replace_range(x as usize..x as usize + 1, "#");
    }

    let hash_count = str_row.matches('#').count();

    // n = 4 if row_str = .#..\#/..#.
    // TODO calc dirs of corners
    // then can determine inside n based on it
    let n = 0;

    n + hash_count
}
