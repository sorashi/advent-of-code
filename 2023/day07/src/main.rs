#![feature(fn_traits)]

use std::{cmp::Ordering, collections::HashMap, io::stdin};

type N = usize;

const CARD_VALUES_GOLD: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];
const CARD_VALUES_SILVER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

struct Hand {
    hand: String,
    bid: N,
    ranking_function: fn(&Self) -> HandType,
    card_values: &'static [char],
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self.ranking_function.call((self, )).cmp(&other.ranking_function.call((&other, ))) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for i in 0..5 {
                    let a = &self.hand[i..i + 1].chars().next().unwrap();
                    let b = &other.hand[i..i + 1].chars().next().unwrap();
                    let a = self.card_values.iter().position(|x| x == a).unwrap();
                    let b = other.card_values.iter().position(|x| x == b).unwrap();
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
    fn get_rank_silver(&self) -> HandType {
        let mut frequencies = HashMap::new();
        for c in self.hand.chars() {
            *frequencies.entry(c).or_insert(0) += 1;
        }
        let mut values: Vec<_> = frequencies.into_values().collect();
        values.sort();
        Self::get_rank_from_sorted_slice(values.as_slice())
    }
    fn get_rank_gold(&self) -> HandType {
        let mut frequencies = HashMap::new();
        for c in self.hand.chars() {
            *frequencies.entry(c).or_insert(0) += 1;
        }
        let jfreq = if let Some(jentry) = frequencies.remove_entry(&'J') {
            jentry.1
        } else {
            0
        };
        let mut values: Vec<_> = frequencies.into_values().collect();
        values.sort();
        match values[..].last_mut() {
            Some(val) => *val += jfreq,
            None => values.push(5),
        }
        Self::get_rank_from_sorted_slice(values.as_slice())
    }
    fn get_rank_from_sorted_slice(sl: &[i32]) -> HandType {
        match sl {
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

fn get_total_winnings(hands: &mut Vec<Hand>) -> N {
    hands.sort_unstable();
    let mut winnings = 0;
    for (idx, hand) in hands.iter().enumerate() {
        winnings += (idx + 1) * hand.bid;
    }
    winnings
}

fn main() {
    let mut hands: Vec<Hand> = stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let spl: Vec<_> = l.split(' ').collect();
            let hand = spl[0].to_string();
            let bid = spl[1].parse::<N>().unwrap();
            Hand { hand, bid, ranking_function: Hand::get_rank_silver, card_values: &CARD_VALUES_SILVER }
        })
        .collect();

    let silver = get_total_winnings(&mut hands);
    println!("silver: {}", silver);

    for hand in &mut hands {
        hand.ranking_function = Hand::get_rank_gold;
        hand.card_values = &CARD_VALUES_GOLD;
    }

    let gold = get_total_winnings(&mut hands);
    println!("gold: {}", gold);
}
