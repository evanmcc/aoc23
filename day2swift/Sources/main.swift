import ArgumentParser
import Foundation

struct Game {
    var number: Int
    var green_max: Int
    var red_max: Int
    var blue_max: Int
}

let RED = 12;
let GREEN = 13;
let BLUE = 14;

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

        var games: [Game] = []

        for line in lines {
            if line == "" { continue }
            let game_rest = line.split(separator: ":")
            let game_no = game_rest[0].split(separator: " ")
            let num = Int(game_no[1])!
            var green_max = 0
            var red_max = 0
            var blue_max = 0

            let try_list = game_rest[1].split(separator: ";")

            for ttry in try_list {
                let t  = ttry.split(separator: ",")
                for cubeset in t {
                    switch cubeset.split(separator: " ") {
                    case let x where x.contains("green"):
                        let int_amt = Int(x[0])!
                        if int_amt > green_max {
                            green_max = int_amt;
                        }
                    case let x where x.contains("blue"):
                        let int_amt = Int(x[0])!
                        if int_amt > blue_max {
                            blue_max = int_amt
                        }
                    case let x where x.contains("red"):
                        let int_amt = Int(x[0])!
                        if int_amt > red_max {
                            red_max = int_amt;
                        }
                    case _:
                        throw fatalError("unmatched thingy");
                    }
                }
            }
            games.append(
              Game(
                number: num,
                green_max: green_max,
                red_max: red_max,
                blue_max: blue_max
              ))
        }

        var total = 0
        var power = 0

        for gm in games {
            if gm.red_max <= RED && gm.green_max <= GREEN && gm.blue_max <= BLUE {
                print("game = ", gm);
                total += gm.number;
            }
            power += gm.red_max * gm.green_max * gm.blue_max;
        }
        print("total: \(total) power \(power)")
    }
}
