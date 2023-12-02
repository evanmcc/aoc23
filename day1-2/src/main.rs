use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_last(line: String) -> String {
    let mut first: char = '!';
    let mut last: char = '!';

    //cheated a bit, last position overlap is allowed
    let replacements = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
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
