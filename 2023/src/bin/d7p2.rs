use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("src/input/d7p1.txt").unwrap();

    let mut hands = input
        .lines()
        .par_bridge()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            let hand = hand.chars().map(|c| c.to_string()).collect::<Vec<_>>();
            let bid = bid.parse::<i32>().unwrap();
            let rank = get_rank(&hand);
            (hand, bid, rank)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        let rank = a.2.cmp(&b.2);
        if rank == std::cmp::Ordering::Equal {
            if is_stronger(&a.0, &b.0) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            rank
        }
    });

    let hands: i32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, b, _))| (i as i32 + 1) * b)
        .sum();

    println!("{:?}", hands);
}

fn is_stronger(a: &[String], b: &[String]) -> bool {
    let strength = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let mut i = 0;
    while a[i] == b[i] {
        i += 1;
    }

    let a = a[i].chars().next().unwrap();
    let b = b[i].chars().next().unwrap();

    let a = strength.iter().position(|&c| c == a).unwrap();
    let b = strength.iter().position(|&c| c == b).unwrap();

    a > b
}

fn get_rank(hand: &[String]) -> i32 {
    let mut jokers = 0;

    let insts = {
        let mut insts: Vec<(char, i32)> = vec![];
        for card in hand {
            let card = card.chars().next().unwrap();

            if card == 'J' {
                jokers += 1
            } else {
                if insts.iter().find(|&&c| c.0 == card).is_some() {
                    insts.iter_mut().find(|&&mut c| c.0 == card).unwrap().1 += 1;
                } else {
                    insts.push((card, 1));
                }
            }
        }

        insts.sort_by(|a, b| a.1.cmp(&b.1));
        insts.reverse();

        insts
    };

    let ranks: Vec<(i32, Vec<i32>)> = vec![
        (6, vec![5]),
        (5, vec![4]),
        (4, vec![3, 2]),
        (3, vec![3]),
        (2, vec![2, 2]),
        (1, vec![2]),
    ];

    let def = (0, vec![]);

    if jokers == 5 {
        return 6;
    }

    ranks
        .iter()
        .find(|(_, ns)| {
            let mut js = jokers.clone();
            ns.iter().enumerate().all(|(i, n)| {
                let b = insts[i].1;
                if b == *n {
                    true
                } else {
                    let diff = *n - b;
                    if js >= diff {
                        js -= 0i32.max(diff);
                        true
                    } else {
                        false
                    }
                }
            })
        })
        .unwrap_or(&def)
        .0
}
