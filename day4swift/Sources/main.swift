import ArgumentParser
import Foundation

struct Card {
    var copies: Int
    var nums: Array<Int>
    var wins: Array<Int>
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

        var linemap: [Int: Card] = [:]

        for line in lines {
            if line == "" { continue }
            let card_rest = line.split(separator: ":")
            let num_str = card_rest[0].split(separator: " ")[1]
            let num = Int(num_str)!

            let nums_wins = card_rest[1].split(separator: "|")
            let nums_strs = nums_wins[0].split(separator: " ")
            let wins_strs = nums_wins[1].split(separator: " ")

            let nums = nums_strs.map { Int($0)! }

            let wins = wins_strs.map { Int($0)! }

            linemap[num] = Card (
              copies: 1,
              nums: nums,
              wins: wins
            )
        }

        for i in 1...linemap.count {
            let card = linemap[i]!

            var subtot = 0;
            for num in card.nums {
                if card.wins.contains(num) {
                    subtot += 1
                }
            }

            let copies = card.copies
            if subtot != 0 {
                for n in 1...subtot {
                    var card2 = linemap[i + n]!
                    card2.copies += copies;
                    linemap[i + n] = card2
                }

            }
        }

        let total = linemap.reduce(0, {$0 + $1.1.copies})

        print("total", total);
    }
}

