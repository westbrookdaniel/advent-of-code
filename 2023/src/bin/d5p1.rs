#[derive(Debug)]
struct Map {
    ranges: Vec<(usize, usize, usize)>,
}

fn main() {
    let input = std::fs::read_to_string("src/input/d5p1.txt").unwrap();

    let mut lines = input.lines();

    let seeds = lines.next().unwrap();
    let seeds = seeds[6..]
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps: Vec<Map> = Vec::new();

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
            maps.push(Map {
                ranges: ranges.clone(),
            });
            ranges.clear();
        }
    });
    maps.push(Map {
        ranges: ranges.clone(),
    });

    // form seeds to .. to location
    let locs = seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for map in &maps {
                let range = map
                    .ranges
                    .iter()
                    .find(|(source, _, range)| return value >= *source && value <= source + range);

                if let Some((source, dest, _)) = range {
                    let diff = value - source;
                    value = dest + diff;
                } else {
                    continue;
                }
            }
            value
        })
        .collect::<Vec<_>>();

    let loc = locs.iter().min().unwrap();

    println!("{:?}", loc);
}
