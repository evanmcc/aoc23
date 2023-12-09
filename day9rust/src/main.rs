use std::fs::File;
use std::io::{BufRead, BufReader};

fn predict(nums: Vec<isize>) -> isize {
    let mut diffs: Vec<isize> = vec![];
    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        diffs.push(diff);
    }
    if diffs.iter().all(|x| *x == 0) {
        *nums.last().unwrap()
    } else {
        let pred = predict(diffs.clone());
        println!("nums {:?} diffs {:?}, pred {}", nums, diffs, pred);
        nums.last().unwrap() + pred
    }
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut total: isize = 0;
    for line in lines {
        let nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let prediction = predict(nums);
        println!("pred {}", prediction);
        total += prediction;
    }
    println!("prediction {}", total);
}
