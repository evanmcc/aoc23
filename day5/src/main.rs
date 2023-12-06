use std::collections::HashMap;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Mode {
    Seed,
    Seed2Soil,
    Soil2Fert,
    Fert2Water,
    Water2Light,
    Light2Temp,
    Temp2Hum,
    Hum2Loc,
}

#[derive(Debug)]
struct Mapping {
    left: usize,
    right: usize,
    len: usize,
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut current = Mode::Seed;

    let mut seeds: Vec<usize> = vec![];

    let mut mappings: HashMap<Mode, Vec<Mapping>> = HashMap::new();

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        } else if line.starts_with("seeds:") {
            let seedlist = line.split(": ").collect::<Vec<&str>>()[1]
                .split_ascii_whitespace()
                .collect::<Vec<&str>>();
            seeds = seedlist.iter().map(|x| x.parse().unwrap()).collect();
        } else if line.starts_with("seed-to-soil") {
            current = Mode::Seed2Soil;
        } else if line.starts_with("soil-to-fertilizer") {
            current = Mode::Soil2Fert;
        } else if line.starts_with("fertilizer-to-water") {
            current = Mode::Fert2Water;
        } else if line.starts_with("water-to-light") {
            current = Mode::Water2Light;
        } else if line.starts_with("light-to-temperature") {
            current = Mode::Light2Temp;
        } else if line.starts_with("temperature-to-humidity") {
            current = Mode::Temp2Hum;
        } else if line.starts_with("humidity-to-location") {
            current = Mode::Hum2Loc;
        } else {
            // all other lines should be map numbers.
            let map_str: Vec<&str> = line.split_ascii_whitespace().collect();
            let map: Vec<usize> = map_str
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let left = map[1];
            let right = map[0];
            let len = map[2];

            match mappings.get_mut(&current) {
                Some(map_vec) => match map_vec.binary_search_by(|x| x.left.cmp(&left)) {
                    Ok(_pos) => {
                        panic!("dupe range")
                    }
                    Err(pos) => map_vec.insert(pos, Mapping { left, right, len }),
                },
                None => {
                    mappings.insert(current, vec![Mapping { left, right, len }]);
                }
            }
        }
    }

    // now we have the map, answer the question
    let mut low_loc = usize::MAX;
    let mut low_seed = 0;
    for seed in seeds.iter() {
        use Mode::*;
        let mut input = *seed;
        println!("seed {}", seed);
        for mode in [
            Seed2Soil,
            Soil2Fert,
            Fert2Water,
            Water2Light,
            Light2Temp,
            Temp2Hum,
            Hum2Loc,
        ]
        .iter()
        {
            let mv = mappings.get(mode).unwrap();
            //println!("mode {:?} vec {:?}", mode, mv);
            for mapping in mv.iter() {
                //println!("map {:?}", mapping);
                if input > (mapping.left + mapping.len) {
                    // too low
                    //println!("cont");
                    continue;
                } else if input >= mapping.left && input < (mapping.left + mapping.len) {
                    // in range
                    let prior = input;
                    input = mapping.right + (input - mapping.left);
                    println!("found {:?} {} {} mapping {:?}", mode, prior, input, mapping);
                    break;
                } else {
                    // missing, leave input as is
                    println!("miss");
                    break;
                }
            }
            if *mode == Hum2Loc {
                println!("loc {}", input);
                if input <= low_loc {
                    low_loc = input;
                    low_seed = *seed;
                }
            }
        }
    }
    println!("lowest loc {} seed {} ", low_loc, low_seed);
}

/*
seed-to-soil map:
soil-to-fertilizer map:
fertilizer-to-water map:
water-to-light map:
light-to-temperature map:
temperature-to-humidity map:
humidity-to-location map:
*/
