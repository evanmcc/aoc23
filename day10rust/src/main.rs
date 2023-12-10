use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   | is a vertical pipe connecting north and south.
   - is a horizontal pipe connecting east and west.
   L is a 90-degree bend connecting north and east.
   J is a 90-degree bend connecting north and west.
   7 is a 90-degree bend connecting south and west.
   F is a 90-degree bend connecting south and east.
   . is ground; there is no pipe in this tile.
   S is the starting position of the .
*/

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn dir_to_pos(d: &Direction) -> (isize, isize) {
    use Direction::*;
    match d {
        North => (0, -1),
        South => (0, 1),
        East => (1, 0),
        West => (-1, 0),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Segment {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

fn seg_to_other(s: Segment, prior: Direction) -> Direction {
    use Direction::*;
    use Segment::*;
    match (s, prior) {
        (NS, South) => South,
        (NS, North) => North,
        (EW, West) => West,
        (EW, East) => East,
        (NW, South) => West,
        (NW, East) => North,
        (NE, South) => East,
        (NE, West) => North,
        (SW, North) => West,
        (SW, East) => South,
        (SE, North) => East,
        (SE, West) => South,
        _ => panic!("bad move, {:?} {:?}", s, &prior),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn char_to_segment(c: char) -> Segment {
    use Segment::*;
    match c {
        '|' => NS,
        '-' => EW,
        'L' => NE,
        'J' => NW,
        '7' => SW,
        'F' => SE,
        '.' => Ground,
        'S' => Start,
        _ => panic!("bad tile"),
    }
}

type Grid = Vec<Vec<Segment>>;

fn main() {
    let infile = std::env::args().nth(1).expect("no input file given.");
    let infh = File::open(std::path::PathBuf::from(infile)).expect("couldn't open file");
    let reader = BufReader::new(infh);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut chars: Vec<Vec<char>> = vec![];
    for line in lines {
        chars.push(line.chars().collect());
    }

    let mut grid: Grid = vec![];
    //reverse the xy direction to make the thinking easier
    for _ in &chars {
        grid.push(vec![])
    }

    let mut start: Pos = Pos { x: 0, y: 0 };
    for i in 0..chars[0].len() {
        for n in 0..chars.len() {
            let seg = char_to_segment(chars[i][n]);
            if seg == Segment::Start {
                start = Pos { x: n, y: i };
            }
            grid[n].push(seg);
        }
    }

    //we now have grid and start position, we need to know what tile type the start is
    // we assume that there are only 2 pipes leading into the start

    let mut dirs: Vec<Direction> = vec![];
    use Direction::*;
    use Segment::*;

    println!("start {:?}", start);
    //north
    println!("north {:?}", &grid[start.x][start.y - 1]);
    match &grid[start.x][&start.y - 1] {
        NS => dirs.push(North),
        SE => dirs.push(North),
        SW => dirs.push(North),
        _ => {}
    }

    //south
    if start.y != 0 {
        println!("south {:?}", &grid[start.x][start.y + 1]);
        match &grid[start.x][&start.y + 1] {
            NS => dirs.push(South),
            NE => dirs.push(South),
            NW => dirs.push(South),
            _ => {}
        }
    }
    //east need max grid limit here, but bleh
    println!("east {:?}", &grid[start.x + 1][start.y]);
    match &grid[start.x + 1][start.y] {
        EW => dirs.push(East),
        SW => dirs.push(East),
        NW => dirs.push(East),
        _ => {}
    }
    //west
    if start.x != 0 {
        match &grid[start.x - 1][start.y] {
            EW => dirs.push(West),
            SE => dirs.push(West),
            NE => dirs.push(West),
            _ => {}
        }
    }

    println!("dirs {:?}", dirs);
    assert!(dirs.len() == 2);
    let (mut cur_type, mut prev_move) = match dirs[..] {
        [North, South] => (NS, North),
        [East, West] => (EW, West),
        [North, East] => (NE, South),
        [North, West] => (NW, South),
        [South, East] => (SE, North),
        [South, West] => (SW, North),
        _ => panic!("bad dirs {:?}", dirs),
    };

    let mut cur = start;
    let mut path_len = 0;
    'steps: loop {
        let other = seg_to_other(cur_type, prev_move);
        let (x, y) = dir_to_pos(&other);
        cur.x = (cur.x as isize + x) as usize;
        cur.y = (cur.y as isize + y) as usize;
        cur_type = grid[cur.x][cur.y];
        path_len += 1;
        // println!(
        //     "next {:?} {:?} prev {:?} move {:?}",
        //     cur, cur_type, prev_move, other
        // );
        prev_move = other;
        if cur_type == Start {
            break 'steps;
        }
    }
    println!("pl {}", path_len / 2);
}
