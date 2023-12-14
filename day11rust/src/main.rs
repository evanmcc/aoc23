use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Galaxy {
    number: usize,
    x: isize,
    y: isize,
}

static EXPANSION: usize = 1_000_000;

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");
    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");
    let reader = BufReader::new(infh);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut chars: Vec<Vec<char>> = vec![];
    let mut ylen = 0;
    for line in lines {
        if ylen == 0 {
            ylen = line.len();
        }
        chars.push(line.chars().collect());
    }

    let mut grid: Grid = vec![];

    for _ in 0..chars[0].len() {
        grid.push(vec![])
    }

    let mut rows: HashSet<usize> = HashSet::from_iter(0..chars.len());
    let mut cols: HashSet<usize> = HashSet::from_iter(0..chars[0].len());

    // first, build the grid
    for x in 0..chars.len() {
        for y in 0..chars[0].len() {
            let c = chars[x][y];
            if c == '#' {
                rows.remove(&x);
                cols.remove(&y);
            }
            grid[x].push(c);
        }
    }

    println!("rows {:?} cols {:?}", rows, cols);

    let mut offset = 0;
    let mut vrows = Vec::from_iter(rows);
    vrows.sort();

    let mut vcols = Vec::from_iter(cols);
    vcols.sort();
    println!("rows {:?} cols {:?}", vrows, vcols);

    let mut num = 0;
    let mut gals: Vec<Galaxy> = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '#' {
                num += 1;
                gals.push(Galaxy {
                    number: num,
                    x: x as isize,
                    y: y as isize,
                });
            }
            print!("{}", grid[x][y]);
        }
        println!();
    }

    for row in vrows {
        for g in &mut gals {
            if g.x >= (row + offset) as isize {
                g.x += (EXPANSION - 1) as isize;
            }
        }
        offset += EXPANSION - 1;
    }

    offset = 0;
    for col in vcols {
        for g in &mut gals {
            if g.y >= (col + offset) as isize {
                g.y += (EXPANSION - 1) as isize;
            }
        }
        offset += EXPANSION - 1;
    }

    println!("{} galaxies", num);
    let mut shortests: Vec<usize> = vec![];
    for (g, g1) in gals.iter().tuple_combinations() {
        let dist = (g.x - g1.x).abs() + (g.y - g1.y).abs();
        println!(
            "dist {} g  {:?} g1 {:?} dx {} dy {}",
            dist,
            g,
            g1,
            (g.x - g1.x).abs(),
            (g.y - g1.y).abs()
        );
        shortests.push(dist as usize);
    }

    println!("total {}", shortests.iter().fold(0, |acc, x| x + acc));
}
