use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

fn char_to_card(c: char) -> Card {
    use Card::*;
    match c {
        'A' => Ace,
        'K' => King,
        'Q' => Queen,
        'J' => Joker,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    let jokers = map.remove_entry(&'J').map_or(0, |(_, x)| x);
    let mut sort_count: Vec<usize> = map.into_values().collect();
    sort_count.sort();
    sort_count.reverse();

    use HandType::*;
    match jokers {
        5 => Five,
        4 => Five, // matches whatever else was there
        3 => match sort_count[0] {
            2 => Five,
            1 => Four,
            _ => panic!("bad hand"),
        },
        2 => match sort_count[0] {
            3 => Five,
            2 => Four,
            1 => Three,
            _ => panic!("bad hand"),
        },
        1 => match sort_count[0] {
            4 => Five,
            3 => Four,
            2 => match sort_count[1] {
                2 => Full,
                1 => Three,
                _ => panic!("bad hand"),
            },
            1 => OnePair,
            _ => panic!("bad hand"),
        },
        0 => match sort_count[0] {
            5 => Five,
            4 => Four,
            3 => match sort_count[1] {
                2 => Full,
                1 => Three,
                _ => panic!("bad hand"),
            },
            2 => match sort_count[1] {
                2 => TwoPair,
                1 => OnePair,
                _ => panic!("bad hand"),
            },
            1 => High,
            _ => panic!("bad hand"),
        },
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
            cards: cards.to_string().chars().map(char_to_card).collect(),
            typ: determine_type(cards.to_string()),
            bid,
        });
    }

    hands.sort();
    let ret = hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        /*
        println!(
            "hand {} rank {} * {} t {:?}",
            hand.str_cards,
            i + 1,
            hand.bid,
            hand.typ
        ); */
        acc + (hand.bid * (i + 1))
    });
    println!("total {}", ret);
}
