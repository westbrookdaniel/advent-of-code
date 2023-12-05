use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let input = std::fs::read_to_string("src/input/d5p1.txt").unwrap();

    let mut lines = input.lines();

    let seeds_ranges = lines.next().unwrap();
    let seeds_ranges = seeds_ranges[6..]
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // move to pairs and expand pairs to ranges
    let mut seeds = Vec::new();
    for i in 0..seeds_ranges.len() / 2 {
        let from = seeds_ranges[i * 2];
        let to = seeds_ranges[i * 2 + 1];
        let range = if from > to { to..=from } else { from..=to };
        seeds.push(range)
    }

    let mut maps: Vec<Vec<(usize, usize, usize)>> = Vec::new();

    let mut from = "";
    let mut to = "";
    let mut ranges = Vec::new();

    lines.for_each(|line| {
        if line.ends_with(" map:") {
            let line = &line[..line.len() - 5];
            (from, to) = line.split_once("-to-").unwrap();
        } else if line.trim() != "" {
            let line = line.trim().split(' ').collect::<Vec<_>>();
            let dest = line[0].parse::<usize>().unwrap();
            let source = line[1].parse::<usize>().unwrap();
            let range = line[2].parse::<usize>().unwrap();
            ranges.push((source, dest, range));
        } else if (from, to) != ("", "") {
            println!("from: {}, to: {}", from, to);
            maps.push(ranges.clone());
        }
    });

    let min = {
        let min = Arc::new(Mutex::new(std::usize::MAX));

        let mut handlers = vec![];

        for seed in seeds {
            for s_start in seed.step_by(100000) {
                let maps = maps.clone();
                let min = min.clone();
                handlers.push(tokio::spawn(async move {
                    for s in s_start..s_start + 100000 {
                        let mut value = s;
                        for map in &maps {
                            let range = map.iter().find(|(source, _, range)| {
                                return value >= *source && value <= source + range;
                            });

                            if let Some((source, dest, _)) = range {
                                let diff = value - source;
                                value = dest + diff;
                            } else {
                                continue;
                            }
                        }
                        let mut min = min.lock().await;
                        if value < *min {
                            *min = value;
                        }
                    }
                }));
            }
        }

        futures::future::join_all(handlers).await;

        let min = min.lock().await;
        *min
    };

    println!("{:?}", min);
}
