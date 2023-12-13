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
    orig: char,
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
        orig: c,
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

    // double the grid so we can flood-fill
    let mut big_grid: Grid = vec![];
    for _ in 0..(grid.len() * 2) {
        big_grid.push(vec![])
    }
    //copy the old grid onto the new one, extending as we go
    for x in 0..big_grid.len() {
        if x % 2 == 0 {
            for y in 0..(grid[0].len() * 2) {
                if y % 2 == 0 {
                    let s = grid[x / 2][y / 2].clone();
                    if s.loc == Loop {
                        big_grid[x].push(s);
                    } else {
                        big_grid[x].push(Seg {
                            mov: Ground,
                            loc: Unknown,
                            orig: '.',
                            is_start: false,
                        });
                    }
                } else {
                    big_grid[x].push(Seg {
                        mov: Ground,
                        loc: Unknown,
                        orig: '.',
                        is_start: false,
                    });
                };
            }
        } else {
            for _ in 0..(grid[0].len() * 2) {
                big_grid[x].push(Seg {
                    loc: Unknown,
                    mov: Ground,
                    orig: '.',
                    is_start: false,
                });
            }
        }
    }

    let up = [SE, SW, NS];
    let down = [NE, NW, NS];
    let right = [NE, SE, EW];
    let left = [NW, SW, EW];
    // now do a pass to connect the loop segments
    let xlen = big_grid.len();
    let ylen = big_grid[0].len();

    for x in 0..big_grid.len() {
        if x % 2 == 1 {
            for y in 0..big_grid[0].len() {
                if y % 2 == 1 {
                    continue;
                }
                if x != 0
                    && right.contains(&big_grid[x - 1][y].mov)
                    && x < xlen - 1
                    && left.contains(&big_grid[x + 1][y].mov)
                {
                    big_grid[x][y] = Seg {
                        mov: EW,
                        loc: Loop,
                        orig: '-',
                        is_start: false,
                    };
                }
            }
        } else {
            for y in 0..big_grid[0].len() {
                if y % 2 == 0 {
                    continue;
                }
                if up.contains(&big_grid[x][y - 1].mov)
                    && y < ylen - 1
                    && down.contains(&big_grid[x][y + 1].mov)
                {
                    big_grid[x][y] = Seg {
                        mov: NS,
                        loc: Loop,
                        orig: '|',
                        is_start: false,
                    };
                }
            }
        }
    }

    // seed the flood fill
    for x in 0..big_grid.len() {
        for y in 0..big_grid[0].len() {
            if (x == 0 || x == xlen - 1) || (y == 0 || y == ylen - 1) {
                if big_grid[x][y].loc == Unknown {
                    big_grid[x][y].loc = Outside;
                }
            }
        }
    }

    // do the flood fill
    let cross: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut changed = true;
    while changed {
        changed = false;
        for ux in 0..big_grid.len() {
            for uy in 0..big_grid[0].len() {
                let x = ux as isize;
                let y = uy as isize;
                if big_grid[ux][uy].loc == Unknown {
                    for (dx, dy) in cross {
                        if x + dx >= 0
                            && y + dy >= 0
                            && x + dx < (xlen - 1) as isize
                            && y + dy < (ylen - 1) as isize
                        {
                            let near = &big_grid[(x + dx) as usize][(y + dy) as usize];
                            if near.loc == Outside {
                                big_grid[ux][uy].loc = Outside;
                                changed = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        // do it backwards as a hack
        for ux in (0..big_grid.len()).rev() {
            for uy in (0..big_grid[0].len()).rev() {
                let x = ux as isize;
                let y = uy as isize;
                if big_grid[ux][uy].loc == Unknown {
                    //println!("{},{}", ux, uy);
                    for (dx, dy) in cross.iter().rev() {
                        if x + dx >= 0
                            && y + dy >= 0
                            && x + dx < (xlen) as isize
                            && y + dy < (ylen) as isize
                        {
                            let near = &big_grid[(x + dx) as usize][(y + dy) as usize];
                            if near.loc == Outside {
                                big_grid[ux][uy].loc = Outside;
                                changed = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    for x in 0..big_grid.len() {
        for y in 0..big_grid[0].len() {
            if big_grid[x][y].loc == Unknown {
                big_grid[x][y].loc = Inside;
            }
        }
    }

    let mut grid3: Grid = vec![];
    for _ in 0..grid.len() {
        grid3.push(vec![])
    }

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            grid3[x].push(big_grid[x * 2][y * 2].clone());
        }
    }

    let mut inside = 0;
    use Placement::*;
    for y in 0..grid3[0].len() {
        for x in 0..grid3.len() {
            match grid3[x][y].loc {
                Loop => print!("{}", grid3[x][y].orig),
                Inside => {
                    print!("I");
                    inside += 1
                }
                Outside => print!("O"),
                _ => print!("."),
            }
        }
        println!("");
    }
    println!("inside count {}", inside);
}
