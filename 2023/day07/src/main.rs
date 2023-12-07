use std::{cmp::Ordering, collections::HashMap, io::stdin};

type n = usize;

const CARD_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

struct Hand {
    hand: String,
    bid: n,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match &self.get_rank().cmp(&other.get_rank()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for i in 0..5 {
                    let a = &self.hand[i..i + 1].chars().next().unwrap();
                    let b = &other.hand[i..i + 1].chars().next().unwrap();
                    let a = CARD_VALUES.iter().position(|x| x == a).unwrap();
                    let b = CARD_VALUES.iter().position(|x| x == b).unwrap();
                    if a == b {
                        continue;
                    }
                    return b.cmp(&a);
                }
                Ordering::Equal
            }
        }
    }
}

impl Hand {
    fn get_rank(&self) -> HandType {
        let mut frequencies = HashMap::new();
        for c in self.hand.chars() {
            *frequencies.entry(c).or_insert(0) += 1;
        }
        let mut values: Vec<_> = frequencies.into_values().collect();
        values.sort();
        match values.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfKind,
            [5] => HandType::FiveOfKind,
            _ => panic!(),
        }
    }
}

fn main() {
    let mut hands: Vec<Hand> = stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let spl: Vec<_> = l.split(' ').collect();
            let hand = spl[0].to_string();
            let bid = spl[1].parse::<n>().unwrap();
            Hand { hand, bid }
        })
        .collect();
    hands.sort();
    let mut silver = 0;
    for (idx, hand) in hands.iter().enumerate() {
        silver += (idx + 1) * hand.bid;
    }
    println!("silver: {}", silver);
}
