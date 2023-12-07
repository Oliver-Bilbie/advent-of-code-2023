use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

impl Card {
    fn get_value(&self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Joker => 1,
            Card::Ten => 11,
            Card::Nine => 10,
            Card::Eight => 9,
            Card::Seven => 8,
            Card::Six => 7,
            Card::Five => 6,
            Card::Four => 5,
            Card::Three => 4,
            Card::Two => 3,
            Card::One => 2,
        }
    }

    fn from_char(c: char) -> Card {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unexpected card: {}", c),
        }
    }
}

#[derive(PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_value(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u16,
    score: u64,
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        let cards = Hand::str_to_cards(&s[..5]);
        let hand_type = Hand::cards_to_type(cards.clone());
        let bid = s[6..].trim().parse::<u16>().unwrap();
        let score = Hand::get_score(&hand_type, &cards);

        Hand {
            hand_type,
            cards,
            bid,
            score,
        }
    }

    fn str_to_cards(s: &str) -> [Card; 5] {
        [
            Card::from_char(s.chars().nth(0).unwrap()),
            Card::from_char(s.chars().nth(1).unwrap()),
            Card::from_char(s.chars().nth(2).unwrap()),
            Card::from_char(s.chars().nth(3).unwrap()),
            Card::from_char(s.chars().nth(4).unwrap()),
        ]
    }

    fn cards_to_type(mut cards: [Card; 5]) -> HandType {
        cards.sort_by(|a, b| b.get_value().cmp(&a.get_value()));

        let mut pairs = HashMap::from([(5, 0), (4, 0), (3, 0), (2, 0), (1, 0)]);
        let mut current_pair_size = 1;
        let mut number_of_jokers = 0;

        for i in 0..4 {
            if cards[i] == Card::Joker {
                number_of_jokers += 1;
            } else if cards[i].get_value() == cards[i + 1].get_value() {
                current_pair_size += 1;
            } else {
                pairs.insert(
                    current_pair_size,
                    pairs.get(&current_pair_size).unwrap() + 1,
                );
                current_pair_size = 1;
            }
        }
        if current_pair_size > 1 {
            pairs.insert(
                current_pair_size,
                pairs.get(&current_pair_size).unwrap() + 1,
            );
        }
        if cards[4] == Card::Joker {
            number_of_jokers += 1;
        }

        let mut hand_type_without_jokers = HandType::HighCard;

        if pairs.get(&5).unwrap() == &1 {
            hand_type_without_jokers = HandType::FiveOfAKind;
        } else if pairs.get(&4).unwrap() == &1 {
            hand_type_without_jokers = HandType::FourOfAKind;
        } else if pairs.get(&3).unwrap() == &1 && pairs.get(&2).unwrap() == &1 {
            hand_type_without_jokers = HandType::FullHouse;
        } else if pairs.get(&3).unwrap() == &1 {
            hand_type_without_jokers = HandType::ThreeOfAKind;
        } else if pairs.get(&2).unwrap() == &2 {
            hand_type_without_jokers = HandType::TwoPair;
        } else if pairs.get(&2).unwrap() == &1 {
            hand_type_without_jokers = HandType::OnePair;
        };

        match hand_type_without_jokers {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => {
                if number_of_jokers == 0 {
                    HandType::FourOfAKind
                } else {
                    HandType::FiveOfAKind
                }
            }
            HandType::FullHouse => HandType::FullHouse,
            HandType::ThreeOfAKind => {
                if number_of_jokers == 0 {
                    HandType::ThreeOfAKind
                } else if number_of_jokers == 1 {
                    HandType::FourOfAKind
                } else {
                    HandType::FiveOfAKind
                }
            }
            HandType::TwoPair => {
                if number_of_jokers == 0 {
                    HandType::TwoPair
                } else {
                    HandType::FullHouse
                }
            }
            HandType::OnePair => {
                if number_of_jokers == 0 {
                    HandType::OnePair
                } else if number_of_jokers == 1 {
                    HandType::ThreeOfAKind
                } else if number_of_jokers == 2 {
                    HandType::FourOfAKind
                } else {
                    HandType::FiveOfAKind
                }
            }
            HandType::HighCard => {
                if number_of_jokers == 0 {
                    HandType::HighCard
                } else if number_of_jokers == 1 {
                    HandType::OnePair
                } else if number_of_jokers == 2 {
                    HandType::ThreeOfAKind
                } else if number_of_jokers == 3 {
                    HandType::FourOfAKind
                } else {
                    HandType::FiveOfAKind
                }
            }
        }
    }

    fn get_score(hand_type: &HandType, cards: &[Card; 5]) -> u64 {
        let base: u64 = 10;
        let mut score = hand_type.get_value() as u64 * base.pow(10);

        for (i, card) in cards.iter().enumerate() {
            let card_value = card.get_value() as u64;
            let order = base.pow(8 - 2 * i as u32);
            score += card_value * order;
        }
        score
    }
}

fn main() {
    let file = File::open("../input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();

    let mut hands: Vec<Hand> = Vec::new();
    let mut total_wininings: u64 = 0;

    for line_content in lines.into_iter() {
        let content = line_content.unwrap();
        let hand = Hand::from_str(&content);
        hands.push(hand);
    }

    hands.sort_by(|a, b| a.score.cmp(&b.score));

    for (index, hand) in hands.iter().enumerate() {
        total_wininings += (hand.bid as u64) * (index as u64 + 1);
    }

    println!("Total wininings: {}", total_wininings);
}
