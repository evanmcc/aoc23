import ArgumentParser
import Foundation

//typealias Grid = [[Character]]

// all this logic has x and y reversed
func scan_remove(y: Int, origin_x: Int, grid: inout [[Character]], xlen: Int) -> Int {
    var local_x = origin_x;
    //scan back left to . or x = 0
    while local_x != 0 && grid[y][local_x - 1].isNumber {
        local_x -= 1;
    }
    //go right collecting and overwriting
    var acc = ""
    while local_x < xlen && grid[y][local_x].isNumber {
        acc += String(grid[y][local_x])
        grid[y][local_x] = "."
        local_x += 1
    }
    //make the int
    let ret = Int(acc)!
    print("found \(ret) at (\(y),\(origin_x))");
    return ret
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

        var grid: [[Character]] = []
        for line in lines {
            if line == "" { continue }
            var chars: [Character] = []
            for c in line { chars.append(c) }
            grid.append(chars)
        }
        var parts: [Int] = []

        // there aren't any symbols on the first or last line, so we can
        //ignore that corner case
        let ylen = grid.count
        let xlen = grid[0].count

        let circle = [
          (-1, -1),
          (0, -1),
          (1, -1),
          (-1, 0),
          (1, 0),
          (-1, 1),
          (0, 1),
          (1, 1),
        ]
        var ratios = 0;

        for y in 0..<ylen {
            for x in 0..<xlen {
                let ch: Character = grid[y][x];
                if (ch.isMathSymbol || ch.isSymbol || ch.isPunctuation) && ch != "." {
                    //scan in a circle, collect and delete any numbers we see to
                    //prevent a double count
                    print("found \(ch) at (\(y),\(x))");

                    var sub_parts: [Int] = [];
                    for (dy, dx) in circle {
                        let lx = (x + dx)
                        let ly = (y + dy)
                        if grid[ly][lx].isNumber {
                            let number = scan_remove(y: ly, origin_x: lx, grid: &grid, xlen: xlen);
                            sub_parts.append(number);
                        }
                    }
                    if sub_parts.count == 2 && ch == "*" {
                        ratios += sub_parts[0] * sub_parts[1];
                    }
                    parts.append(contentsOf: sub_parts);
                }
            }
        }
        let tot = parts.reduce(0, {x, y in x + y})
        print("total: \(tot), ratio: \(ratios)");

    }
}
