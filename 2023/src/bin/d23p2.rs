use std::collections::{HashMap, VecDeque};

use pathfinding::matrix::Matrix;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, i32);

impl Pos {
    fn successors(&self, grid: &Matrix<char>, visited: &mut HashMap<(i32, i32), bool>) -> Vec<Pos> {
        let Pos(y, x, n) = self.clone();

        visited.insert((y, x), true);

        let s = [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .iter()
            .filter(|p| {
                // cant visit if already in t
                if *visited.get(&(p.0, p.1)).or(Some(&false)).unwrap() {
                    return false;
                }

                match grid.get((p.0 as usize, p.1 as usize)) {
                    Some('>') => true,
                    Some('<') => true,
                    Some('^') => true,
                    Some('v') => true,
                    // Some('>') => x < p.1,
                    // Some('<') => x > p.1,
                    // Some('^') => y > p.0,
                    // Some('v') => y < p.0,
                    Some('.') => true,
                    Some('#') => false,
                    None => false,
                    a => panic!("unexpected {:?}", a),
                }
            })
            .map(|p| {
                let (y, x) = p;
                Pos(*y, *x, n + 1)
            })
            .collect();

        // println!("successors for {:?} are {:?}", self, s);

        s
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d23p1.txt").unwrap();

    let grid = Matrix::from_rows(
        input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>()),
    )
    .unwrap();

    let goal = Pos(grid.rows as i32 - 2, grid.columns as i32 - 2, 0);

    let mut lengths = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((Pos(0, 1, 0), "0".to_string()));

    let mut visited_maps = HashMap::new();

    while queue.len() > 0 {
        let (p, id) = queue.pop_front().unwrap();

        let next = {
            let mut visited = visited_maps.entry(id.clone()).or_insert(HashMap::new());

            if p.0 == goal.0 && p.1 == goal.1 {
                println!("one way took {:?} steps", p.2 + 1);
                lengths.push(p.2 + 1);
            }

            p.successors(&grid, &mut visited)
        };

        for (i, n) in next.iter().enumerate() {
            let new_id = format!("{}_{}", id.clone(), i);
            let prev = visited_maps.get(&id).unwrap();
            visited_maps.insert(new_id.clone(), prev.clone());
            queue.push_back((n.clone(), new_id));
        }

        visited_maps.remove(&id);
    }

    // println!("{:?}", lengths);
    let longest = lengths.iter().max().unwrap();
    println!("{:?}", longest);

    // println!("{}", input);

    // for point in last.clone().unwrap() {
    //     println!("{:?}", point);
    // }

    // for (y, line) in input.trim().lines().enumerate() {
    //     for (x, c) in line.trim().chars().enumerate() {
    //         if last
    //             .clone()
    //             .unwrap()
    //             .iter()
    //             .find(|p| p.1 == x as i32 && p.0 == y as i32)
    //             .is_some()
    //         {
    //             print!("O");
    //             continue;
    //         }
    //         print!("{}", c);
    //     }
    //     println!();
    // }
}
