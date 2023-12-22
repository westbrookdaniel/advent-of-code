use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    // 1 is on the ground
    z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    // this is sudo 3D space, each xyz point is 1x1x1 cube
    start: Point,
    end: Point,
    id: usize,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(",").map(|s| s.parse::<i32>().unwrap());
        Self {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
    }
}

impl Brick {
    fn will_fall_on(&self, other: &Self) -> bool {
        // includes is already resting on
        self.distance_above(other) > 0 && self.is_aligned(other)
    }

    fn distance_above(&self, other: &Self) -> i32 {
        let self_highest = self.end.z.max(self.start.z);
        let other_lowest = other.start.z.min(other.end.z);
        self_highest - other_lowest
    }

    fn is_aligned(&self, other: &Self) -> bool {
        // do the 2 2d liens (xy) overlap?
        let x_min = self.start.x.min(self.end.x);
        let x_max = self.start.x.max(self.end.x);
        let other_x_min = other.start.x.min(other.end.x);
        let other_x_max = other.start.x.max(other.end.x);

        let x_overlap = x_min <= other_x_max && other_x_min <= x_max;

        let y_min = self.start.y.min(self.end.y);
        let y_max = self.start.y.max(self.end.y);
        let other_y_min = other.start.y.min(other.end.y);
        let other_y_max = other.start.y.max(other.end.y);

        let y_overlap = y_min <= other_y_max && other_y_min <= y_max;

        x_overlap && y_overlap
    }

    fn fall_on(&mut self, other: &Self) {
        let distance = self.distance_above(other);
        // println!("falling {} on {}", self.id, other.id);
        // println!("distance: {}", distance - 1);
        // println!();
        self.start.z -= distance - 1;
        self.end.z -= distance - 1;
    }

    fn fall_to_ground(&mut self) {
        let self_highest = self.end.z.max(self.start.z);
        let distance_to_ground = self_highest - 1;
        // println!("falling {} to ground", self.id);
        // println!("distance: {}", distance_to_ground);
        // println!();
        self.start.z -= distance_to_ground;
        self.end.z -= distance_to_ground;
    }

    fn fall(&mut self, bricks: &Vec<Brick>) {
        let mut bricks = bricks.clone();
        bricks.sort_by(sort_highest_first);

        let mut did_fall = false;

        for brick in bricks {
            if self.id != brick.id && self.will_fall_on(&brick) {
                self.fall_on(&brick);
                did_fall = true;
                break;
            }
        }

        if !did_fall {
            self.fall_to_ground();
        }
    }

    fn is_resting_on(&self, other: &Self) -> bool {
        self.distance_above(other) == 1 && self.is_aligned(other)
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d22p1.txt").unwrap();

    let mut bricks = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.trim().split_once("~").unwrap();
            let start = Point::from(start);
            let end = Point::from(end);
            // let id = (i as u8 + 65) as char;
            Brick { start, end, id: i }
        })
        .collect::<Vec<_>>();

    bricks.sort_by(sort_highest_first);

    // print_top_to_bottom(&bricks);
    // println!();

    let l = bricks.len();
    let mut last_round = None;
    let mut bricks = bricks.clone();
    // TODO: could optimise this
    for i in 0..l {
        println!("round {} of {}", i, l);

        // if the same as last round, we are done
        if let Some(last_round) = &last_round {
            if last_round == &bricks {
                break;
            }
        }

        let mut fallen_bricks = vec![];
        for brick in &bricks {
            let mut brick = brick.clone();
            brick.fall(&bricks);
            fallen_bricks.push(brick);
        }

        last_round = Some(bricks.clone());
        bricks = fallen_bricks.clone();
    }

    // print_top_to_bottom(&bricks);
    // println!();

    // println!("c: {:?}", bricks[2]);
    // println!("b: {:?}", bricks[1]);
    // println!("is c on b? {}", bricks[2].is_resting_on(&bricks[1]));
    // println!();

    let mut n = 0;
    for brick in &bricks {
        // println!("can we remove {}?", brick.id);
        let can_remove = can_safely_remove(brick, &bricks);
        // println!("{}: {:?}", brick.id, can_remove);
        // println!();
        if can_remove {
            n += 1;
        }
    }

    println!("{}", n);
}

// fn print_top_to_bottom(bricks: &Vec<Brick>) {
//     let mut bricks = bricks.clone();
//     bricks.sort_by(sort_highest_first);
//     for brick in &bricks {
//         println!("{}: {:?}", brick.id, brick);
//     }
//     println!();
// }

fn can_safely_remove(brick: &Brick, bricks: &Vec<Brick>) -> bool {
    // if has no bricks resting on it, true
    let resting_on = bricks
        .iter()
        .filter(|b| b.id != brick.id)
        .filter(|b| b.is_resting_on(brick))
        .collect::<Vec<_>>();

    if resting_on.len() == 0 {
        return true;
    }

    resting_on
        .iter()
        .map(|brick| {
            // amount sitting on
            let resting_on = bricks
                .iter()
                .filter(|b| b.id != brick.id)
                .filter(|b| brick.is_resting_on(b))
                .collect::<Vec<_>>();

            resting_on.len()
        })
        .all(|r| r > 1)
}

fn sort_highest_first(a: &Brick, b: &Brick) -> Ordering {
    let a_dist = a.distance_above(&b);
    let b_dist = b.distance_above(&a);
    b_dist.cmp(&a_dist)
}
