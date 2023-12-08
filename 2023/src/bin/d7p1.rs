fn main() {
    let input = std::fs::read_to_string("src/input/d7p1.txt").unwrap();

    let mut hands = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            let hand = hand.chars().map(|c| c.to_string()).collect::<Vec<_>>();
            let bid = bid.parse::<i32>().unwrap();
            let rank = get_rank(&hand);
            (hand, bid, rank)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        // compare on rank and then stronger (if rank is equal)
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

    // let hands: i32 = hands
    //     .iter()
    //     .enumerate()
    //     .map(|(i, (_, b, _))| (i as i32 + 1) * b)
    //     .sum();

    // println!("{:?}", hands);

    let hands = hands.iter().enumerate().collect::<Vec<_>>();

    for (i, (h, b, s)) in hands {
        println!("{:?}", (i + 1, h.join(""), b, s));
    }
}

fn is_stronger(a: &[String], b: &[String]) -> bool {
    let strength = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
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

fn get_rank(hand: &[String]) -> u32 {
    let mut rank: u32 = 0;

    let insts = {
        let mut insts: Vec<(char, u32)> = vec![];
        for card in hand {
            let card = card.chars().next().unwrap();
            if insts.iter().find(|&&c| c.0 == card).is_some() {
                insts.iter_mut().find(|&&mut c| c.0 == card).unwrap().1 += 1;
            } else {
                insts.push((card, 1));
            }
        }
        insts
    };

    if insts.len() == 1 {
        rank += 6;
    } else if insts.iter().any(|&c| c.1 == 4) {
        rank += 5;
    } else if insts.iter().any(|&c| c.1 == 3) && insts.iter().any(|&c| c.1 == 2) {
        rank += 4;
    } else if insts.iter().any(|&c| c.1 == 3) {
        rank += 3;
    } else if insts.iter().filter(|&c| c.1 == 2).count() == 2 {
        rank += 2;
    } else if insts.iter().any(|&c| c.1 == 2) {
        rank += 1;
    }

    rank
}
