use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    copies: usize,
    nums: Vec<usize>,
    wins: Vec<usize>,
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut linemap: HashMap<usize, Card> = HashMap::new();

    for line in lines.iter() {
        let card_rest: Vec<&str> = line.split(':').collect();
        let num_str = card_rest[0].split_ascii_whitespace().collect::<Vec<&str>>()[1];
        let num: usize = num_str.parse().unwrap();

        let nums_wins: Vec<&str> = card_rest[1].split('|').collect();
        let nums_strs: Vec<&str> = nums_wins[0].split_ascii_whitespace().collect();
        let wins_strs: Vec<&str> = nums_wins[1].split_ascii_whitespace().collect();

        let nums: Vec<usize> = nums_strs
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let wins: Vec<usize> = wins_strs
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        linemap.insert(
            num,
            Card {
                copies: 1,
                nums: nums.clone(),
                wins: wins.clone(),
            },
        );
    }

    for i in 1..=linemap.len() {
        let card: &Card = linemap.get_mut(&i).unwrap();

        let mut subtot = 0;
        for num in card.nums.iter() {
            if card.wins.contains(num) {
                subtot += 1;
            }
        }
        let copies = card.copies;
        for n in 1..=subtot {
            let card2 = linemap.get_mut(&(i + n)).unwrap();
            card2.copies += copies;
        }
    }

    let total = linemap.iter().fold(0, |acc, (_, c)| acc + c.copies);

    println!("total {}", total);
}
