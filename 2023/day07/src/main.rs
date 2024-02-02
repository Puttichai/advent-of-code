use std::env;
use std::fs;
use std::slice::Iter;
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");
    let contents: String = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");
    let result1: u64 = part1(&contents);
    println!("result1 = {result1}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CamelCard {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

// For iterating over CamelCard. See also this SO answer
// https://stackoverflow.com/a/21376984
impl CamelCard {
    pub fn iterator() -> Iter<'static, CamelCard> {
        static CARDS: [CamelCard; 13] = [CamelCard::C2,
                                         CamelCard::C3,
                                         CamelCard::C4,
                                         CamelCard::C5,
                                         CamelCard::C6,
                                         CamelCard::C7,
                                         CamelCard::C8,
                                         CamelCard::C9,
                                         CamelCard::T,
                                         CamelCard::J,
                                         CamelCard::Q,
                                         CamelCard::K,
                                         CamelCard::A];
        CARDS.iter()
    }
}

fn camel_card_from_string(card_c: char) -> CamelCard {
    match card_c {
        '2' => CamelCard::C2,
        '3' => CamelCard::C3,
        '4' => CamelCard::C4,
        '5' => CamelCard::C5,
        '6' => CamelCard::C6,
        '7' => CamelCard::C7,
        '8' => CamelCard::C8,
        '9' => CamelCard::C9,
        'T' => CamelCard::T,
        'J' => CamelCard::J,
        'Q' => CamelCard::Q,
        'K' => CamelCard::K,
        'A' => CamelCard::A,
        _ => panic!("Wrond card character {card_c}"),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<CamelCard>,
}

fn compute_hand_type_from_cards(cards: &[CamelCard]) -> HandType {
    let mut num_pairs: usize = 0;
    let mut num_threes: usize = 0;
    for check_card in CamelCard::iterator() {
        let num_cards: usize = cards.iter()
            .filter(|&card| card == check_card)
            .count();
        if num_cards == 5 {
            return HandType::FiveOfAKind;
        } else if num_cards == 4 {
            return HandType::FourOfAKind;
        } else if num_cards == 3 {
            num_threes += 1;
        } else if num_cards == 2 {
            num_pairs += 1;
        }
    }
    if num_threes > 0 {
        return if num_pairs > 0 { HandType::FullHouse } else { HandType::ThreeOfAKind };
    }
    if num_pairs > 0 {
        return if num_pairs > 1 { HandType::TwoPair } else { HandType::OnePair };
    }
    HandType::HighCard
}

fn init_hand_from_string(cards_str: &str) -> Hand {
    let current_cards: Vec<CamelCard> = cards_str
        .chars()
        .map(|x| camel_card_from_string(x))
        .collect::<Vec<_>>();
    Hand {
        hand_type: compute_hand_type_from_cards(&current_cards),
        cards: current_cards,
    }
}

fn part1(contents: &str) -> u64 {
    let mut players: Vec<(Hand, u64)> = vec![];
    for line in contents.lines() {
        let hand_bid_str: Vec<&str> = line
            .trim()
            .split_whitespace()
            .collect();
        players.push(
            (init_hand_from_string(hand_bid_str[0]),
             hand_bid_str[1].parse::<u64>().unwrap())
        );
    }
    players.sort_by(|player_a, player_b| {
        let a: &Hand = &player_a.0;
        let b: &Hand = &player_b.0;
        let ord: Ordering = a.hand_type.cmp(&b.hand_type);
        if ord == Ordering::Equal {
            for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                let cmp_result: Ordering = card_a.cmp(&card_b);
                if cmp_result != Ordering::Equal {
                    return cmp_result;
                }
            }
            panic!("");
        }
        else {
            return ord;
        }
    });

    // println!("players={:?}", players);
    players
        .iter()
        .enumerate()
        .map(|(index, elem)| {
            ((index as u64) + 1) * elem.1
        })
        .sum()
}
