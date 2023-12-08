use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Node {
    left: String,
    right: String,
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut instruction_list = "";
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in lines.iter() {
        if instruction_list.is_empty() {
            instruction_list = line;
        } else if line.is_empty() {
            continue;
        } else {
            let name = &line[0..3].to_string();
            let left = &line[7..10].to_string();
            let right = &line[12..15].to_string();

            nodes.insert(
                name.clone(),
                Node {
                    left: left.clone(),
                    right: right.clone(),
                },
            );
        }
    }

    let mut current: &Node = nodes.get("AAA").unwrap();
    let mut steps = 1;
    'outer: loop {
        for inst in instruction_list.chars() {
            let next = match inst {
                'R' => current.right.clone(),
                'L' => current.left.clone(),
                _ => panic!("bad instruction"),
            };
            println!("inst {} next {} steps {}", inst, next, steps);
            if next == "ZZZ" {
                println!("steps {}", steps);
                break 'outer;
            } else {
                current = nodes.get(&next).unwrap();
                steps += 1;
            }
        }
    }
}
