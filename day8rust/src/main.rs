use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let l = nums[0];
    let r = lcm(&nums[1..]);
    l * r / gcd(l, r)
}

fn gcd(l: usize, r: usize) -> usize {
    if r == 0 {
        return l;
    }
    gcd(r, l % r)
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut instruction_list = "";
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut starts: Vec<String> = vec![];

    for line in lines.iter() {
        if instruction_list.is_empty() {
            instruction_list = line;
        } else if line.is_empty() {
            continue;
        } else {
            let name = &line[0..3].to_string();
            let left = &line[7..10].to_string();
            let right = &line[12..15].to_string();

            if name.ends_with('A') {
                starts.push(name.clone());
            }
            nodes.insert(
                name.clone(),
                Node {
                    left: left.clone(),
                    right: right.clone(),
                },
            );
        }
    }

    println!("starts {:?}", starts);
    let mut currents: Vec<&Node> = vec![];
    for s in starts.iter() {
        currents.push(nodes.get(s).unwrap());
    }

    let mut steps: Vec<usize> = vec![];
    for cur in currents.iter_mut() {
        let mut step_count = 0;
        'outer: loop {
            for inst in instruction_list.chars() {
                let next = match inst {
                    'R' => cur.right.clone(),
                    'L' => cur.left.clone(),
                    _ => panic!("bad instruction"),
                };
                *cur = nodes.get(&next).unwrap();
                step_count += 1;
                if next.ends_with('Z') {
                    steps.push(step_count);
                    break 'outer;
                }
            }
        }
    }
    let res = lcm(&steps);
    println!("steps {}", res);
}
