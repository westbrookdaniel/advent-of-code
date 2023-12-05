use rayon::prelude::*;

#[derive(Debug)]
struct Map {
    entries: Vec<(usize, usize, usize)>,
}

impl Map {
    fn locate(&self, seed: usize) -> usize {
        let range = self
            .entries
            .iter()
            .find(|(s, _, r)| seed >= *s && seed <= s + r);

        if let Some((source, dest, _)) = range {
            dest + (seed - source)
        } else {
            seed
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d5p1.txt").unwrap();

    let mut lines = input.lines();

    let seeds = lines.next().unwrap();
    let seeds = seeds[6..]
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Box<_>>();

    let mut maps: Vec<Map> = Vec::new();

    let mut from = "";
    let mut to = "";
    let mut entries = Vec::new();

    lines.for_each(|line| {
        if line.ends_with(" map:") {
            let line = &line[..line.len() - 5];
            (from, to) = line.split_once("-to-").unwrap();
        } else if line.trim() != "" {
            let line = line.trim().split(' ').collect::<Vec<_>>();
            let dest = line[0].parse::<usize>().unwrap();
            let source = line[1].parse::<usize>().unwrap();
            let range = line[2].parse::<usize>().unwrap();
            entries.push((source, dest, range));
        } else if (from, to) != ("", "") {
            maps.push(Map {
                entries: entries.clone(),
            });
            entries.clear();
        }
    });
    maps.push(Map {
        entries: entries.clone(),
    });

    let min = seeds
        .chunks(2)
        .map(|s| s[0]..s[0] + s[1])
        .map(|seed| {
            seed.into_par_iter()
                .map(|s| get_loc(s, &maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("{:?}", min);
}

fn get_loc(seed: usize, maps: &[Map]) -> usize {
    maps.iter().fold(seed, |acc, map| map.locate(acc))
}
