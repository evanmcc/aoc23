use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Row {
    pattern: Vec<States>,
    groups: Vec<usize>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum States {
    Working,
    Broken,
    Unknown,
    PlacedWorking,
    PlacedBroken,
}

#[derive(Debug, Clone)]
enum PatStates {
    MaybeWorking,
    MaybeBroken,
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");
    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");
    let reader = BufReader::new(infh);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    use PatStates::*;
    use States::*;
    let mut rows: Vec<Row> = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let [pattern_str, groups_str] = line.split_ascii_whitespace().collect::<Vec<&str>>()[..]
        else {
            todo!()
        };
        let groups: Vec<usize> = groups_str
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let pattern: Vec<States> = pattern_str
            .chars()
            .map(|c| match c {
                '?' => Unknown,
                '.' => Working,
                '#' => Broken,
                _ => todo!(),
            })
            .collect();
        let unf_pattern = (0..5).flat_map(|_| pattern.clone()).collect();
        let unf_groups = (0..5).flat_map(|_| groups.clone()).collect();
        rows.push(Row {
            pattern: unf_pattern,
            groups: unf_groups,
        });
    }

    let mut arrangements: Vec<usize> = vec![];
    for row in rows {
        let pats: Vec<Vec<PatStates>> = row
            .groups
            .iter()
            .map(|g| {
                let mut v = vec![MaybeWorking];
                v.extend((0..*g).map(|_| MaybeBroken));
                v.push(MaybeWorking);
                v
            })
            .collect();

        // for each location in the fixed pattern (row), try to fit the group. Once the group is
        // fit, try to fit the next group.  if we succeed fitting all groups, save off the final
        // pattern to arrangements if it is unique.

        let arrs = fit(-1, pats.len(), &pats, &row.pattern);
        println!("{} {:?}", arrs.len(), row,);
        // for a in &arrs {
        //     a.iter().for_each(|x| match x {
        //         Working => print!("."),
        //         Broken => print!("#"),
        //         _ => {}
        //     });
        //     println!();
        // }
        arrangements.push(arrs.len());
    }

    println!("arrangements: {:?}", arrangements.iter().sum::<usize>());
}

fn fit(
    start: isize,
    count: usize,
    groups: &[Vec<PatStates>],
    pattern: &[States],
) -> Vec<Vec<States>> {
    use States::*;
    if groups.is_empty() {
        let fin: Vec<States> = pattern
            .iter()
            .map(|x| match x {
                &Working | &PlacedWorking | &Unknown => Working,
                &Broken | &PlacedBroken => Broken,
            })
            .collect();
        if count_groups(&fin[..]) == count {
            return vec![fin];
        } else {
            return vec![];
        }
    }

    let group = &groups[0];
    //println!("group {:?}", group);
    let rest = &groups[1..];
    let mut ret: Vec<Vec<States>> = vec![];
    let mut i: isize = start;
    while i <= pattern.len() as isize {
        let (idx, grp) = if i == -1 {
            //println!("first");
            (0, &group[1..])
        } else if i == pattern.len() as isize {
            //println!("last");
            (i as usize, &group[..group.len() - 1])
        } else {
            (i as usize, &group[..])
        };
        if let Some((fitted, endpos)) = overlay(idx, grp, pattern) {
            let fits = fit(endpos, count, rest, &fitted);
            for f in fits {
                if !ret.contains(&f) {
                    ret.push(f);
                }
            }
        }

        i += 1;
    }
    ret
}

fn overlay(pos: usize, group: &[PatStates], pattern: &[States]) -> Option<(Vec<States>, isize)> {
    use PatStates::*;
    use States::*;

    let mut ret = vec![];
    let mut endpos = 0;
    ret.extend(pattern);
    for (idx, ps) in group.iter().enumerate() {
        if pos + idx == pattern.len() {
            if idx == group.len() - 1 {
                break;
            } else {
                return None;
            }
        }
        match ps {
            MaybeWorking => match ret[pos + idx] {
                PlacedWorking | Working | Unknown => ret[pos + idx] = PlacedWorking,
                _ => return None,
            },
            MaybeBroken => match ret[pos + idx] {
                Broken | Unknown => {
                    ret[pos + idx] = PlacedBroken;
                    endpos = pos + idx + 1;
                }
                _ => return None,
            },
        }
    }

    //println!("ret {:?}", ret);
    Some((ret, endpos as isize))
}

fn count_groups(pattern: &[States]) -> usize {
    use States::*;
    let mut in_group = false;
    let mut ret = 0;
    for s in pattern {
        match s {
            Broken | PlacedBroken => {
                if in_group {
                    continue;
                } else {
                    in_group = true;
                    ret += 1;
                }
            }
            Working => {
                if in_group {
                    in_group = false;
                }
            }
            _ => todo!(),
        }
    }
    ret
}
