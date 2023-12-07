use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Jack = 10,
    Ten = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
}

fn char_to_card(c: char) -> Card {
    use Card::*;
    match c {
        'A' => Ace,
        'K' => King,
        'Q' => Queen,
        'J' => Jack,
        'T' => Ten,
        '9' => Nine,
        '8' => Eight,
        '7' => Seven,
        '6' => Six,
        '5' => Five,
        '4' => Four,
        '3' => Three,
        '2' => Two,
        _ => panic!("bad card"),
    }
}

// define order for sort
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1,
}

#[derive(Eq, PartialEq)]
struct Hand {
    str_cards: String,
    cards: Vec<Card>,
    typ: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.typ.cmp(&other.typ);
        if cmp == Ordering::Equal && self.cards != other.cards {
            for i in 0..self.cards.len() {
                let card_cmp = self.cards[i].cmp(&other.cards[i]);
                if card_cmp == Ordering::Equal {
                    continue;
                } else {
                    return card_cmp;
                }
            }
            unreachable!();
        } else {
            cmp
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn determine_type(hand: String) -> HandType {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut sort_count: Vec<usize> = map.into_values().collect();
    sort_count.sort();
    sort_count.reverse();
    match sort_count[0] {
        5 => HandType::Five,
        4 => HandType::Four,
        3 => match sort_count[1] {
            2 => HandType::Full,
            1 => HandType::Three,
            _ => panic!("bad hand"),
        },
        2 => match sort_count[1] {
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            _ => panic!("bad hand"),
        },
        1 => HandType::High,
        _ => panic!("bad hand"),
    }
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut hands: Vec<Hand> = vec![];
    for line in lines.iter() {
        let bid_parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let cards = bid_parts[0];
        let bid: usize = bid_parts[1].parse().unwrap();
        hands.push(Hand {
            str_cards: cards.to_string(),
            cards: cards.to_string().chars().map(|c| char_to_card(c)).collect(),
            typ: determine_type(cards.to_string()),
            bid,
        });
    }

    hands.sort();
    let ret = hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        println!("hand {} rank {} bid {}", hand.str_cards, i + 1, hand.bid);
        acc + (hand.bid * (i + 1))
    });
    println!("total {}", ret);
}
