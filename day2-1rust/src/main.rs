use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Game {
    number: usize,
    green_max: usize,
    red_max: usize,
    blue_max: usize,
}

static RED: usize = 12;
static GREEN: usize = 13;
static BLUE: usize = 14;

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let mut games: Vec<Game> = vec![];

    for line_res in reader.lines() {
        match line_res {
            Ok(line) => {
                let game_rest: Vec<&str> = line.split(':').collect();
                let game_no: Vec<&str> = game_rest[0].split_ascii_whitespace().collect();
                let num = game_no[1].parse::<usize>().unwrap();
                let mut green_max = 0;
                let mut red_max = 0;
                let mut blue_max = 0;

                let try_list: Vec<&str> = game_rest[1].split(';').collect();

                for ttry in try_list.iter() {
                    let t: Vec<&str> = ttry.split(',').collect();
                    for cubeset in t.iter() {
                        match cubeset.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                            [amt, "green"] => {
                                let int_amt = amt.parse::<usize>().unwrap();
                                if int_amt > green_max {
                                    green_max = int_amt;
                                }
                            }
                            [amt, "blue"] => {
                                let int_amt = amt.parse::<usize>().unwrap();
                                if int_amt > blue_max {
                                    blue_max = int_amt;
                                }
                            }
                            [amt, "red"] => {
                                let int_amt = amt.parse::<usize>().unwrap();
                                if int_amt > red_max {
                                    red_max = int_amt;
                                }
                            }
                            _ => {
                                panic!("unmatched thingy");
                            }
                        }
                    }
                }
                games.push(Game {
                    number: num,
                    green_max,
                    red_max,
                    blue_max,
                });
            }
            _ => {
                panic!("oh no!")
            }
        }
    }

    let mut total: usize = 0;
    let mut power: usize = 0;

    for gm in games.iter() {
        if gm.red_max <= RED && gm.green_max <= GREEN && gm.blue_max <= BLUE {
            println!("game = {:?}", gm);
            total += gm.number;
        }
        power += gm.red_max * gm.green_max * gm.blue_max;
    }
    println!("total: {} power {}", total, power);
}
