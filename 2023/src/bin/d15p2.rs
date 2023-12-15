use std::collections::HashMap;

fn main() {
    let inputs = std::fs::read_to_string("src/input/d15p1.txt").unwrap();

    let mut boxes: HashMap<i32, Vec<(&str, i32)>> = HashMap::new();

    for input in inputs.split(',') {
        let mut input = input.trim().split(['-', '=']);
        let label = input.next().unwrap();
        let hash = hash(label);
        let lens_or_empty = input.next().unwrap();

        match lens_or_empty {
            "" => {
                let b = boxes.entry(hash).or_insert(Vec::new());

                let found = b.iter().position(|(l, _)| *l == label);
                if let Some(i) = found {
                    b.remove(i);
                }
            }
            _ => {
                let b = boxes.entry(hash).or_insert(Vec::new());

                let found = b.iter().position(|(l, _)| *l == label);
                if let Some(i) = found {
                    let _ = std::mem::replace(&mut b[i], (label, lens_or_empty.parse().unwrap()));
                } else {
                    b.push((label, lens_or_empty.parse().unwrap()));
                }
            }
        };
    }

    let mut out = 0;
    for (key, b) in boxes {
        for (i, item) in b.iter().enumerate() {
            out += (key + 1) * item.1 * (i as i32 + 1);
        }
    }

    println!("{:?}", out);
}

fn hash(input: &str) -> i32 {
    let mut v: i32 = 0;
    for char in input.trim().chars() {
        v += char as i32;
        v = v * 17;
        v = v % 256;
    }
    v
}
