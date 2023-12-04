use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;

// all this logic has x and y reversed
fn scan_remove(y: usize, origin_x: usize, grid: &mut Grid, xlen: usize) -> usize {
    let mut local_x = origin_x;
    //scan back left to . or x = 0
    while local_x != 0 && grid[y][local_x - 1].is_ascii_digit() {
        local_x -= 1;
    }
    //go right collecting and overwriting
    let mut acc = "".to_string();
    while local_x < xlen && grid[y][local_x].is_ascii_digit() {
        acc += &grid[y][local_x].to_string();
        grid[y][local_x] = '.';
        local_x += 1;
    }
    //make the int
    let ret: usize = acc.parse().unwrap();
    println!("found {} at ({},{})", ret, y, origin_x);
    ret
}

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");

    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");

    let reader = BufReader::new(infh);

    let mut grid: Grid = vec![];

    // build the grid
    for line_res in reader.lines() {
        match line_res {
            Ok(line) => {
                let line_vec: Vec<char> = line.chars().collect();
                grid.push(line_vec);
            }
            _ => {
                panic!("oh noes");
            }
        }
    }

    let mut parts: Vec<usize> = vec![];

    // there aren't any symbols on the first or last line, so we can
    //ignore that corner case
    let ylen = grid.len();
    let xlen = grid[0].len();

    let circle = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for y in 0..ylen {
        for x in 0..xlen {
            let ch: char = grid[y][x];
            if ch.is_ascii_punctuation() && ch != '.' {
                //scan in a circle, collect and delete any numbers we see to
                //prevent a double count
                println!("found {} at ({},{})", ch, y, x);
                for (dy, dx) in circle.iter() {
                    let lx = (x as isize + dx) as usize;
                    let ly = (y as isize + dy) as usize;
                    if grid[ly][lx].is_ascii_digit() {
                        let number = scan_remove(ly, lx, &mut grid, xlen);
                        parts.push(number);
                    }
                }
            }
        }
    }
    let tot: usize = parts.iter().sum();
    println!("total: {}", tot);
}
