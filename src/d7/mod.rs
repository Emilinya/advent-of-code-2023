use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

use crate::utils;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    hand_string: String,
    hand_type: HandType,
    use_joker: bool,
    bet: u32,
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.hand_string == other.hand_string && self.hand_type == other.hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }

        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (c1, c2) in self.hand_string.chars().zip(other.hand_string.chars()) {
            match (self.map_card(&c1), other.map_card(&c2)) {
                (Some(v1), Some(v2)) if v1 != v2 => {
                    return v1.cmp(v2);
                }
                (Some(_), Some(_)) => {}
                _ => panic!("Got non-card chars! {:?}, {:?}", c1, c2),
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    const NORMAL_CARD_VALUE_MAP: phf::Map<char, u8> = phf::phf_map! {
        'A' => 12, 'K' => 11, 'Q' => 10, 'J' => 9, 'T' => 8,
        '9' => 7, '8' => 6, '7' => 5, '6' => 4, '5' => 3,
        '4' => 2, '3' => 1, '2' => 0,
    };

    const JOKER_CARD_VALUE_MAP: phf::Map<char, u8> = phf::phf_map! {
        'A' => 12, 'K' => 11, 'Q' => 10, 'T' => 9, '9' => 8,
        '8' => 7, '7' => 6, '6' => 5, '5' => 4, '4' => 3,
        '3' => 2, '2' => 1, 'J' => 0,
    };

    fn get_hand_type(hand_string: &str, use_joker: bool) -> HandType {
        let mut hand_map = HashMap::<char, u8>::new();
        for c in hand_string.chars() {
            hand_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        // We always want the joker to be the most common card
        if use_joker && hand_map.contains_key(&'J') {
            let joker_count = *hand_map.get(&'J').unwrap();

            if joker_count as usize != hand_string.len() {
                let (max_card, _) = hand_map
                    .iter()
                    .filter(|(k, _)| **k != 'J')
                    .max_by_key(|(_, v)| *v)
                    .expect("there is no max card‽");
                hand_map.entry(*max_card).and_modify(|v| *v += joker_count);
            } else {
                hand_map.insert('A', joker_count);
            }

            hand_map.remove(&'J');
        }

        match hand_map.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if *hand_map.values().max().unwrap() == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if *hand_map.values().max().unwrap() == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("hand_map.len() is 0 or > 5! {:?}", hand_map),
        }
    }

    fn new(hand_string: &str, use_joker: bool, bet: u32) -> Self {
        Hand {
            hand_string: hand_string.to_owned(),
            hand_type: Hand::get_hand_type(hand_string, use_joker),
            use_joker,
            bet,
        }
    }

    fn map_card(&self, card: &char) -> Option<&u8> {
        if self.use_joker {
            Hand::JOKER_CARD_VALUE_MAP.get(card)
        } else {
            Hand::NORMAL_CARD_VALUE_MAP.get(card)
        }
    }
}

fn get_total_winnings(filename: &str, use_joker: bool) -> u32 {
    let mut hands: Vec<Hand> = utils::read_lines(filename)
        .map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            [hand, bet_str] => Hand::new(
                hand,
                use_joker,
                bet_str
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("malformed bet_str: {}", bet_str)),
            ),
            _ => panic!("malformed line: {:?}", line),
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bet)
        .sum()
}

fn test() {
    assert_eq!(get_total_winnings("src/d7/test_input.dat", false), 6440);
    assert_eq!(get_total_winnings("src/d7/test_input.dat", true), 5905);
}

pub fn test_final() {
    assert_eq!(get_total_winnings("src/d7/full_input.dat", false), 251136060);
    assert_eq!(get_total_winnings("src/d7/full_input.dat", true), 249400220);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_total_winnings("src/d7/full_input.dat", false);
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_total_winnings("src/d7/full_input.dat", true);
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
