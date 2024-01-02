use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {:?}", solution(&input));
}

fn solution(input: &str) -> u32 {
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        hands.push(Hand::new(line));
    }
    hands.sort_by(|a, b| {
        if b.hand_type.cmp(&a.hand_type).is_eq() {
            for i in 0..5 {
                if b.cards[i].cmp(&a.cards[i]).is_ne() {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
            return a.cards[0].cmp(&b.cards[0]);
        } else {
            return b.hand_type.cmp(&a.hand_type);
        }
    });
    println!("This is the hands: {:?}", hands);
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bet * (i as u32 + 1))
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: Vec<Card>) -> HandType {
        let mut hash: HashMap<Card, u32> = HashMap::new();
        for card in cards {
            hash.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }

        match hash.len() {
            1 => HandType::FiveKind,
            2 => {
                if hash.values().any(|x| *x == 4) {
                    HandType::FourKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if hash.values().any(|x| *x == 3) {
                    HandType::ThreeKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,

            _ => HandType::HighCard,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(input: &u8) -> Card {
        match input {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: u32,
}
impl Hand {
    fn new(input: &str) -> Hand {
        let cards_string: String = input.split(" ").take(1).flat_map(|s| s.chars()).collect();
        let bet_string: String = input
            .split(" ")
            .skip(1)
            .take(1)
            .flat_map(|s| s.chars())
            .collect();

        let cards: Vec<Card> = cards_string
            .as_bytes()
            .into_iter()
            .map(|b| Card::from_char(b))
            .collect();

        let bet: u32 = bet_string.trim().parse().unwrap();
        let hand_type = HandType::from_cards(cards.clone());

        Hand {
            cards,
            hand_type,
            bet,
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        assert_eq!(solution(input), 6440);
    }
}
