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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Placement {
    Loop,
    Inside,
    Outside,
    Unknown,
}

#[derive(Debug, Clone)]
struct Seg {
    mov: Segment,
    loc: Placement,
    inside: Vec<Direction>,
    is_start: bool,
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

fn char_to_segment(c: char) -> Seg {
    use Segment::*;
    let mov = match c {
        '|' => NS,
        '-' => EW,
        'L' => NE,
        'J' => NW,
        '7' => SW,
        'F' => SE,
        '.' => Ground,
        'S' => Start,
        _ => panic!("bad tile"),
    };
    Seg {
        mov,
        loc: Placement::Unknown,
        inside: vec![],
        is_start: c == 'S',
    }
}

type Grid = Vec<Vec<Seg>>;

fn trace<F>(start: Pos, start_type: Segment, start_prev_move: Direction, grid: &mut Grid, mut f: F)
where
    F: FnMut(Pos, Segment, &mut Grid),
{
    let mut cur = start;
    let mut cur_type = start_type;
    let mut prev_move = start_prev_move;
    'steps: loop {
        let other = seg_to_other(cur_type, prev_move);
        let (x, y) = dir_to_pos(&other);
        cur.x = (cur.x as isize + x) as usize;
        cur.y = (cur.y as isize + y) as usize;
        let seg = grid[cur.x][cur.y].clone();
        cur_type = seg.mov;
        f(cur, cur_type, grid);
        prev_move = other;
        if cur == start {
            break 'steps;
        }
    }
}

fn update_from_nearby(ux: usize, uy: usize, grid: &mut Grid) {
    use Direction::*;
    use Placement::*;

    if grid[ux][uy].loc == Loop {
        return;
    }

    let x = ux as isize;
    let y = uy as isize;
    let xlen = grid.len() as isize;
    let ylen = grid[0].len() as isize;

    let cross = [(0, -1, North), (0, 1, South), (1, 0, East), (-1, 0, West)];
    for (dx, dy, dir) in cross {
        if x + dx >= 0 && y + dy >= 0 && x + dx < xlen && y + dy < ylen {
            let near = &grid[(x + dx) as usize][(y + dy) as usize];
            match near.loc {
                Outside => {
                    grid[ux][uy].loc = Outside;
                    break;
                }
                Inside => {
                    grid[ux][uy].loc = Inside;
                    break;
                }
                Loop => {
                    if near.inside.contains(&dir) {
                        // this is the bad case actually, since we're
                        // looking in dir and it's on the other side!
                        grid[ux][uy].loc = Outside;
                    } else {
                        println!(
                            "near {},{} {:?}  {:?} dir {:?}",
                            x + dx,
                            y + dy,
                            near.inside,
                            near.mov,
                            dir
                        );
                        grid[ux][uy].loc = Inside;
                    }
                    break;
                }
                _ => continue,
            }
        }
    }
}

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
    for _ in 0..chars[0].len() {
        grid.push(vec![])
    }

    let mut start: Pos = Pos { x: 0, y: 0 };
    for i in 0..chars.len() {
        for n in 0..chars[0].len() {
            let seg = char_to_segment(chars[i][n]);
            if seg.mov == Segment::Start {
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
    //println!("north {:?}", &grid[start.x][start.y - 1]);
    if start.y != 0 {
        match &grid[start.x][&start.y - 1].mov {
            NS => dirs.push(North),
            SE => dirs.push(North),
            SW => dirs.push(North),
            _ => {}
        }
    }

    //south
    //println!("south {:?}", &grid[start.x][start.y + 1]);
    match &grid[start.x][&start.y + 1].mov {
        NS => dirs.push(South),
        NE => dirs.push(South),
        NW => dirs.push(South),
        _ => {}
    }

    //east need max grid limit here, but bleh
    //println!("east {:?}", &grid[start.x + 1][start.y]);
    match &grid[start.x + 1][start.y].mov {
        EW => dirs.push(East),
        SW => dirs.push(East),
        NW => dirs.push(East),
        _ => {}
    }
    //west
    if start.x != 0 {
        match &grid[start.x - 1][start.y].mov {
            EW => dirs.push(West),
            SE => dirs.push(West),
            NE => dirs.push(West),
            _ => {}
        }
    }

    println!("dirs {:?}", dirs);
    assert!(dirs.len() == 2);
    let (cur_type, prev_move) = match dirs[..] {
        [North, South] => (NS, North),
        [East, West] => (EW, West),
        [North, East] => (NE, South),
        [North, West] => (NW, South),
        [South, East] => (SE, North),
        [South, West] => (SW, North),
        _ => panic!("bad dirs {:?}", dirs),
    };

    //set the start tile, now marked by a bool, to it's grid type
    grid[start.x][start.y].mov = cur_type;

    let mut path_len = 0;
    trace(start, cur_type, prev_move, &mut grid, |p, _, grd| {
        grd[p.x][p.y].loc = Placement::Loop;
        path_len += 1
    });
    println!("pl {}", path_len / 2);

    // pick the y midpoint and trace in from X 0 till you hit a side, then trace the circle
    let mid = (grid.len() / 2) as usize;
    let mut y = 0;
    while grid[mid][y].loc != Loop {
        y += 1;
        continue;
    }
    let (prev_move2, mut last_inside) = match grid[mid][y].mov {
        EW => {
            grid[mid][y].inside.push(South);
            (West, vec![South])
        }
        SE => {
            grid[mid][y].inside.extend([South, East]);
            (North, vec![South, East])
        }
        SW => {
            grid[mid][y].inside.extend([South, West]);
            (North, vec![South, West])
        }
        _ => panic!("bad segment"),
    };
    //now that we have one, extend it around the loop
    trace(
        Pos { x: mid, y },
        grid[mid][y].mov,
        prev_move2,
        &mut grid,
        |p, t, g| {
            // last
            let next_inside: Vec<Direction> = match t {
                NS => {
                    if last_inside.contains(&East) {
                        vec![East]
                    } else {
                        vec![West]
                    }
                }
                EW => {
                    if last_inside.contains(&North) {
                        vec![North]
                    } else {
                        vec![South]
                    }
                }
                NW | SE => {
                    if last_inside.contains(&North) || last_inside.contains(&West) {
                        vec![North, West]
                    } else {
                        vec![South, East]
                    }
                }
                NE | SW => {
                    if last_inside.contains(&North) || last_inside.contains(&East) {
                        vec![North, East]
                    } else {
                        vec![South, West]
                    }
                }
                _ => panic!(
                    "bad last_inside {:?} loc {:?}",
                    last_inside, g[p.x][p.y].mov
                ),
            };
            g[p.x][p.y].inside.extend(next_inside.clone());
            //println!("{:?}", g[p.x][p.y].inside);
            last_inside = next_inside;
        },
    );

    let mut inside = 0;
    use Placement::*;
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            update_from_nearby(x, y, &mut grid);
            match grid[x][y].loc {
                Loop => match grid[x][y].inside[..] {
                    [North] => print!("^"),
                    [South] => print!("v"),
                    [East] => print!(">"),
                    [West] => print!("<"),

                    [North, West] => print!("F"),
                    [South, West] => print!("L"),
                    [North, East] => print!("7"),
                    [South, East] => print!("J"),
                    _ => print!("."),
                },
                Inside => {
                    print!("I");
                    inside += 1
                }
                Outside => print!("."),
                _ => print!("U"),
            }
        }
        println!("");
    }
    println!("inside count {}", inside);
}
