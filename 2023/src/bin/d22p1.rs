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
    fn distance_above(&self, other: &Self) -> i32 {
        let self_highest = self.end.z.min(self.start.z);
        let other_lowest = other.start.z.max(other.end.z);
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

    // fn will_fall_on(&self, other: &Self) -> bool {
    //     // includes is already resting on
    //     self.distance_above(other) > 0 && self.is_aligned(other)
    // }

    // fn fall_on(&mut self, other: &Self) {
    //     let distance = self.distance_above(other);
    //     // println!("falling {} on {}", self.id, other.id);
    //     // println!("distance: {}", distance - 1);
    //     // println!();
    //     self.start.z -= distance - 1;
    //     self.end.z -= distance - 1;
    // }

    // fn fall_to_ground(&mut self) {
    //     self.fall_to(0);
    // }

    fn fall_to(&mut self, z: i32) {
        let self_lowest = self.end.z.min(self.start.z);
        let distance_to_ground = self_lowest - (z + 1);
        // println!("falling {} to {}", self.id, z);
        // println!("distance: {}", distance_to_ground);
        // println!();
        self.start.z -= distance_to_ground;
        self.end.z -= distance_to_ground;
    }

    fn fall(&mut self, z_map: &Vec<Vec<i32>>) {
        let points = points_in_brick(self);
        let z_map_points = points
            .iter()
            .map(|point| {
                let x = point.x as usize;
                let y = point.y as usize;
                z_map[y][x]
            })
            .collect::<Vec<_>>();

        let highest_z = z_map_points.iter().max().unwrap();
        // println!("i am: {:?}", letter(self.id));
        // println!("points: {:?}", points);
        // println!("highest z: {}", highest_z);
        // println!("FROM: {:?}", self);
        self.fall_to(*highest_z);
        // println!("TO: {:?}", self);
        // println!();
    }

    fn is_resting_on(&self, other: &Self) -> bool {
        // println!("{}: d {}, al {}", letter(self.id), self.distance_above(other), self.is_aligned(other));
        self.distance_above(other) == 1 && self.is_aligned(other)
    }
}

fn empty_z_map(bricks: &Vec<Brick>) -> Vec<Vec<i32>> {
    // 2d grid of highest z location for each x,y
    let largest_x = bricks.iter().map(|b| b.end.x).max().unwrap();
    let largest_y = bricks.iter().map(|b| b.end.y).max().unwrap();
    vec![vec![0; largest_x as usize + 1]; largest_y as usize + 1]
}

fn add_to_z_map(z_map: &mut Vec<Vec<i32>>, brick: &Brick) {
    let points = points_in_brick(brick);
    for point in points {
        let z = point.z;
        let x = point.x as usize;
        let y = point.y as usize;
        if z_map[y][x] < z {
            z_map[y][x] = z;
        }
    }
}

fn points_in_brick(brick: &Brick) -> Vec<Point> {
    let mut points = vec![];
    for x in brick.start.x..=brick.end.x {
        for y in brick.start.y..=brick.end.y {
            for z in brick.start.z..=brick.end.z {
                points.push(Point { x, y, z });
            }
        }
    }
    points
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

    // println!("==BEFORE====");
    // print_bricks(&bricks);

    let bricks = {
        bricks.sort_by(sort_highest_first);
        bricks.reverse();

        let mut z_map = empty_z_map(&bricks);

        let mut fallen_bricks = vec![];
        for brick in &bricks {
            let mut brick = *brick;
            brick.fall(&z_map);
            fallen_bricks.push(brick);
            add_to_z_map(&mut z_map, &brick);
        }

        fallen_bricks
    };

    // println!("==AFTER=====");
    // print_bricks(&bricks);

    // print_top_to_bottom(&bricks);
    // println!();

    // println!("c: {:?}", bricks[2]);
    // println!("b: {:?}", bricks[1]);
    // println!("is c on b? {}", bricks[2].is_resting_on(&bricks[1]));
    // println!();

    let mut n = 0;
    for brick in &bricks {
        // println!("can we remove {}?", letter(brick.id));
        let can_remove = can_safely_remove(brick, &bricks);
        // println!("{:?}", can_remove);
        // println!();
        if can_remove {
            n += 1;
        }
    }

    println!("{}", n);
}

fn print_bricks(bricks: &Vec<Brick>) {
    let mut bricks = bricks.clone();
    bricks.sort_by(sort_highest_first);
    for brick in &bricks {
        println!("{}: {:?}", letter(brick.id), brick);
    }
    println!();
}

fn can_safely_remove(brick: &Brick, bricks: &Vec<Brick>) -> bool {
    // println!("can we remove {}?", letter(brick.id));

    // if has no bricks resting on it, true
    let resting_on = bricks
        .iter()
        .filter(|b| b.id != brick.id)
        .filter(|b| b.is_resting_on(brick))
        .collect::<Vec<_>>();

    if resting_on.len() == 0 {
        // println!("{} has nothing resting on it", letter(brick.id));
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

            // println!(
            //     "{} is resting on: {:?}",
            //     letter(brick.id),
            //     resting_on.iter().map(|b| letter(b.id)).collect::<Vec<_>>()
            // );

            resting_on.len()
        })
        .all(|r| r > 1)
}

fn sort_highest_first(a: &Brick, b: &Brick) -> Ordering {
    let a_dist = a.distance_above(&b);
    let b_dist = b.distance_above(&a);
    b_dist.cmp(&a_dist)
}

fn letter(id: usize) -> char {
    (id as u8 + 65) as char
}
