use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_last(line: String) -> String {
    let mut first: char = '\0';
    let mut last: char = '\0';

    let replacements = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let line1 = replacements
        .iter()
        .fold(line, |acc, (from, to)| acc.replace(from, to));

    for ch in line1.chars() {
        if ch.is_ascii_digit() {
            first = ch;
            break;
        }
    }
    let rev_line1: String = line1.chars().rev().collect();

    for ch in rev_line1.chars() {
        if ch.is_ascii_digit() {
            last = ch;
            break;
        }
    }
    println!("line1 {}", line1);
    first.to_string() + &last.to_string()
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);
    let mut total: usize = 0;
    for line_res in reader.lines() {
        match line_res {
            Ok(line) => {
                let fl = first_last(line.clone());
                let fl_int = fl.parse::<usize>().unwrap();
                println!("line {} fl {} int {}", line, fl, fl_int);
                total += fl_int;
            }
            _ => {
                panic!("oh no!")
            }
        }
    }

    println!("sum total: {}", total);
}
