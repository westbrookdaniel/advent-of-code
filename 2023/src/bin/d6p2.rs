fn main() {
    let time: i64 = 61677571;
    let dist: i64 = 430103613071150;

    let (min, max) = solve_held(time as f64, dist as f64);
    let min = min.floor() as i64;
    let max = max.ceil() as i64;
    let diff = max - min;

    println!("{:?}", diff - 1);
}

fn solve_held(time: f64, dist: f64) -> (f64, f64) {
    let top = -time + f64::sqrt((time * time) - 4.0 * dist);
    let min = -(top / 2.0);

    let top = -time - f64::sqrt((time * time) - 4.0 * dist);
    let max = -(top / 2.0);

    (min, max)
}
