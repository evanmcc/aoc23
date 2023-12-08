use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_last(line: String) -> String {
    let mut first: char = '\0';
    let mut last: char = '\0';

    for ch in line.chars() {
        if ch.is_ascii_digit() {
            first = ch;
            break;
        }
    }
    let rev_line: String = line.chars().rev().collect();

    for ch in rev_line.chars() {
        if ch.is_ascii_digit() {
            last = ch;
            break;
        }
    }

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
