fn main() {
    // time, distance
    // let pairs = [(7, 9), (15, 40), (30, 200)];
    // Time:        61     67     75     71
    // Distance:   430   1036   1307   1150
    let pairs = [(61, 430), (67, 1036), (75, 1307), (71, 1150)];

    let out = pairs
        .iter()
        .map(|&(time, dist)| {
            let (min, max) = solve_held(time as f32, dist as f32);
            let min = min.floor() as i32;
            let max = max.ceil() as i32;
            let diff = max - min;

            diff - 1
        })
        .product::<i32>();

    println!("{:?}", out);
}

// fn solve_dist(time: f32, held: f32) -> f32 {
//     held * (time - held)
// }

fn solve_held(time: f32, dist: f32) -> (f32, f32) {
    let top = -time + f32::sqrt((time * time) - 4.0 * dist);
    let min = -(top / 2.0);

    let top = -time - f32::sqrt((time * time) - 4.0 * dist);
    let max = -(top / 2.0);

    (min, max)
}
