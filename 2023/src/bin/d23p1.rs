use pathfinding::matrix::Matrix;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, i32, Vec<(i32, i32)>);

impl Pos {
    fn successors(&self, grid: &Matrix<char>) -> Vec<(Pos, u32)> {
        let Pos(y, x, n, t) = self.clone();

        let s = [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .iter()
            .filter(|p| {
                // cant visit if already in t
                if t.contains(&(p.0, p.1)) {
                    return false;
                }

                match grid.get((p.0 as usize, p.1 as usize)) {
                    // Some('>') => true,
                    // Some('<') => true,
                    // Some('^') => true,
                    // Some('v') => true,
                    Some('>') => x < p.1,
                    Some('<') => x > p.1,
                    Some('^') => y > p.0,
                    Some('v') => y < p.0,
                    Some('.') => true,
                    Some('#') => false,
                    None => false,
                    a => panic!("unexpected {:?}", a),
                }
            })
            .map(|p| {
                let (y, x) = p;
                let mut t = t.clone();
                t.push((*y, *x));
                (Pos(*y, *x, n, t), 1)
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

    let goal = Pos(grid.rows as i32 - 2, grid.columns as i32 - 2, 0, vec![]);

    let mut lens = vec![];
    // let mut last = None;

    loop {
        let result = dijkstra(
            &Pos(0, 1, 0, vec![]),
            |p| p.successors(&grid),
            |p| p.0 == goal.0 && p.1 == goal.1 && !lens.contains(&(p.2 + 1)),
        );

        if let Some(result) = result {
            println!("{:?} so far", lens.len());
            lens.push(result.1 as i32 + 1);
            // last = Some(result.0);
            continue;
        }

        break;
    }

    // println!("{:?}", lens);
    let longest = lens.iter().max().unwrap();
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
